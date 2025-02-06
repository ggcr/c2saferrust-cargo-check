#![feature(backtrace)]
#![feature(rustc_private)]
#![feature(let_else)]
#![feature(min_specialization)]
#![feature(array_windows)]
#![feature(once_cell)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_mir_dataflow;
extern crate rustc_data_structures;
extern crate rustc_index;
extern crate rustc_serialize;

extern crate rustc_version;
extern crate cargo_metadata;
extern crate petgraph;
extern crate smallvec;
extern crate itertools;
extern crate regex;

use rustc_middle::ty::TyCtxt;

mod visitor;
mod utils;
mod subgraph;
mod liveness;
mod bb_wrapper;
mod dominators;
mod block_visitor;
mod decompose;

use decompose::{decompose, extract_func_deps};
use liveness::do_dataflow_analysis;

use utils::*;

#[derive(Debug, Clone)]
pub struct SlicerConfig {
    pub output_file: String,
    pub max_lines: usize,
    pub min_lines: usize,
    pub dataflow_only: bool,
    // If dataflow_only is true, then input_file must be specified
    pub input_file: Option<String>,
    pub append: bool,
}

impl Default for SlicerConfig {
    fn default() -> Self {
        SlicerConfig {
            output_file: "output.json".to_string(),
            max_lines: 150,
            min_lines: 10,
            dataflow_only: false,
            input_file: None,
            append: false,
        }
    }
}

/// Returns the "default sysroot" that Slicer will use if no `--sysroot` flag is set.
/// Should be a compile-time constant.
pub fn compile_time_sysroot() -> Option<String> {
    // option_env! is replaced to a constant at compile time
    if option_env!("RUSTC_STAGE").is_some() {
        // This is being built as part of rustc, and gets shipped with rustup.
        // We can rely on the sysroot computation in librustc.
        return None;
    }

    // For builds outside rustc, we need to ensure that we got a sysroot
    // that gets used as a default. The sysroot computation in librustc would
    // end up somewhere in the build dir.
    // Taken from PR <https://github.com/Manishearth/rust-clippy/pull/911>.
    let home = option_env!("RUSTUP_HOME").or(option_env!("MULTIRUST_HOME"));
    let toolchain = option_env!("RUSTUP_TOOLCHAIN").or(option_env!("MULTIRUST_TOOLCHAIN"));
    Some(match (home, toolchain) {
        (Some(home), Some(toolchain)) => format!("{}/toolchains/{}", home, toolchain),
        _ => option_env!("RUST_SYSROOT")
            .expect("To build Slicer without rustup, set the `RUST_SYSROOT` env var at build time")
            .to_owned(),
    })
}

pub fn analyze(&tcx: &TyCtxt<'_>, config: SlicerConfig) {

    if config.dataflow_only && config.input_file.is_none() {
        panic!("If dataflow_only is true, then input_file must be specified");
    }

    let mut json_output = if ! config.dataflow_only {
        decompose(tcx, config.clone())
    }
    else {
        // Read the input file
        let input_file = config.clone().input_file.unwrap();
        let input_json = std::fs::read_to_string(input_file).unwrap();
        let input_json: serde_json::Value = serde_json::from_str(&input_json).unwrap();
        let serde_json::Value::Array(inner_array) = input_json else {
            panic!("Expected an array of functions in the input file");
        };
        inner_array
    };
    
    for func in json_output.iter_mut() {
        let func_defid = func["func_defid"].as_str().unwrap().to_string().clone();

        let mir_body = get_mir_body_by_defid_string(tcx, func_defid.clone());
        if mir_body.is_none() {
            continue;
        }
        let mir_body = mir_body.unwrap();

        let func_deps = extract_func_deps(tcx, func);
        if func_deps.is_none() {
            continue;
        }
        let func_deps = func_deps.unwrap();
        func["calls"] = func_deps["calls"].clone();
        func["globals"] = func_deps["globals"].clone();
        func["imports"] = func_deps["imports"].clone();
        
        // Why does this not need to be mutable?
        let mut chunks = func["chunks"].as_array_mut().unwrap();
        // If the function has no chunks
        if chunks.len() == 0 {
            continue;
        }
        for chunk in chunks.iter_mut() {
            let chunk_span = chunk["span"].as_str().unwrap().to_string();
            let chunk_span = get_span_from_span_str(tcx, chunk_span);
            let dataflow_json = do_dataflow_analysis(tcx, chunk_span, &mir_body);
            if dataflow_json.is_none() {
                println!("WARNING - dataflow analysis failed for chunk {:?}", chunk_span);
                chunk["live_in"] = serde_json::Value::Array(vec![]);
                chunk["live_out"] = serde_json::Value::Array(vec![]);
                continue;
            }
            let dataflow_json = dataflow_json.unwrap();
            // Add the dataflow_json to the chunk
            chunk["live_in"] = dataflow_json["live_in"].clone();
            chunk["live_out"] = dataflow_json["live_out"].clone();
        }
    }

    let old_json_output = if config.append {
        // Read the old output file
        let output_file = config.clone().output_file;
        let output_json = std::fs::read_to_string(output_file).unwrap();
        let output_json: serde_json::Value = serde_json::from_str(&output_json).unwrap();
        let serde_json::Value::Array(inner_array) = output_json else {
            panic!("Expected an array of functions in the input file");
        };
        inner_array
    }
    else {
        vec![]
    };

    // Filtering the old_json_output for duplicates
    let mut old_json_output = old_json_output.iter()
                    .filter(|old_func|
                            ! json_output.iter().any(|new_func| new_func["func_defid"] == old_func["func_defid"])
                    ).map(|func| func.clone())
                    .collect::<Vec<serde_json::Value>>();
    
    old_json_output
                    .extend(json_output.iter().map(|func| func.clone()));

    // Write the json to a file
    let stringified_json = serde_json::to_string_pretty(&old_json_output).unwrap();
    std::fs::write(config.clone().output_file, stringified_json).unwrap();
}