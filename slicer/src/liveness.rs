use rustc_middle::mir::{Local, Statement, StatementKind, Body, Location, BasicBlock, Terminator};
use rustc_mir_dataflow::{Analysis, AnalysisDomain, GenKillAnalysis, GenKill, CallReturnPlaces, storage::always_storage_live_locals};
use rustc_index::bit_set::BitSet;
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

use serde_json::{self, json};
use itertools::Itertools;
use regex::Regex;

use crate::utils::*;

// We want to know what are the variables *used* and *defined* in some given sequence of blocks
// We find all the variables that are LIVE_IN for the first block,
// and all the variables that are LIVE_OUT for the last block

#[derive(Clone)]
pub struct LivenessAnalysis {
    always_live_locals: BitSet<Local>
}

impl LivenessAnalysis {
	pub fn new(always_live_locals: BitSet<Local>) -> Self {
		LivenessAnalysis { always_live_locals }
	}
}

impl<'tcx> AnalysisDomain<'tcx> for LivenessAnalysis {

	type Domain = BitSet<Local>;
	const NAME: &'static str = "liveness_analysis";

	fn bottom_value(&self, body: &Body<'tcx>) -> Self::Domain {
		// bottom = live
		BitSet::new_empty(body.local_decls.len())
	}

	fn initialize_start_block(
        &self, body: &Body<'tcx>, on_entry: &mut Self::Domain) {
		assert_eq!(
            body.local_decls.len(), 
            self.always_live_locals.domain_size()
        );
        for local in self.always_live_locals.iter() {
            on_entry.insert(local);
        }
		// for local in body.vars_and_temps_iter() {
	}
}

impl<'tcx> GenKillAnalysis<'tcx> for LivenessAnalysis {
    type Idx = Local;

    fn statement_effect(
        &self,
        trans: &mut impl GenKill<Self::Idx>,
        stmt: &Statement<'tcx>,
        _: Location,
    ) {
        match stmt.kind {
            StatementKind::StorageLive(l) => trans.gen(l),
            StatementKind::StorageDead(l) => trans.kill(l),
            _ => (),
        }
    }

    fn terminator_effect(
        &self,
        _trans: &mut impl GenKill<Self::Idx>,
        _: &Terminator<'tcx>,
        _: Location,
    ) {
        // Terminators have no effect
    }

    fn call_return_effect(
        &self,
        _trans: &mut impl GenKill<Self::Idx>,
        _block: BasicBlock,
        _return_places: CallReturnPlaces<'_, 'tcx>,
    ) {
        // Nothing to do when a call returns successfully
    }
}

pub fn compute_liveness<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &'tcx Body<'tcx>,
    block1_index: BasicBlock,
    block2_index: BasicBlock
) -> (Vec<Local>, Vec<Local>) {
    let always_live_locals: BitSet<Local> = always_storage_live_locals(&body);
    let mut liveness_results = LivenessAnalysis::new(always_live_locals)
                        .into_engine(tcx, body)
                        .iterate_to_fixpoint()
                        .into_results_cursor(body);

    liveness_results.seek_to_block_start(block1_index);
    let liveness1 = liveness_results.get();
    let mut live_in: Vec<Local> = Vec::new();
    for local in liveness1.iter() {
        live_in.push(local);
    }
    liveness_results.seek_to_block_end(block2_index);
    let liveness2 = liveness_results.get();
    let mut live_out: Vec<Local> = Vec::new();
    for local in liveness2.iter() {
        live_out.push(local);
    }
    (live_in, live_out)
}

pub fn do_dataflow_analysis<'tcx>(tcx: TyCtxt<'tcx>, chunk_span: Span, mir_body: &'tcx Body<'tcx>) -> Option<serde_json::Value> {

    let source_map = tcx.sess.source_map();

    let pre_block = get_last_bb_before_start_of_span( &chunk_span, &mir_body);
    let block1_index = get_first_bb_after_start_of_span(&chunk_span, &mir_body);
    let block2_index = get_last_bb_before_end_of_span( &chunk_span, &mir_body);
    let post_block = get_first_bb_after_end_of_span(&chunk_span, &mir_body);
    
    if block1_index.is_none() || block2_index.is_none() {
        return None;
    }

    let selected_pre_statements = if pre_block.is_some() {
        select_statements_within_span(pre_block.unwrap(), &chunk_span, &mir_body)
    } else {
        Vec::new()
    };
    let selected_post_statements = if post_block.is_some() {
        select_statements_within_span(post_block.unwrap(), &chunk_span, &mir_body)
    } else {
        Vec::new()
    };
    let (live_pre, dead_pre) = get_live_dead_from_statements(selected_pre_statements);
    let (live_post, dead_post) = get_live_dead_from_statements(selected_post_statements);

    let (live_in, live_out) = compute_liveness(tcx, mir_body, block1_index.unwrap(), block2_index.unwrap());

    let live_in: Vec<Local> = live_in.iter()
                                     .chain(dead_pre.iter())
                                     .filter(|x| !live_pre.contains(x))
                                     .unique().cloned().collect();
    let live_out: Vec<Local> = live_out.iter()
                                       .chain(live_post.iter())
                                       .filter(|x| !dead_post.contains(x))
                                       .unique().cloned().collect();

    let chunk_source = source_map.span_to_snippet(chunk_span).unwrap();
    
    let mut live_in_str = Vec::new();
    let mut live_out_str = Vec::new();
    for (i, local) in live_in.iter().chain(live_out.iter()).enumerate() {
        if let Some(symbol) = local_to_symbol(*local, mir_body) {
            // Check with Regex if symbol appears in func_body
            // Otherwise we don't need to pass it as an argument
            let re = Regex::new(&format!(r"\b{}\b", symbol)).unwrap();
            if !re.is_match(&chunk_source) {
                continue;
            }
            let local_decl = &mir_body.local_decls[*local];
            let mutability = local_decl.mutability;
            let mut_string: String = if mutability == rustc_middle::mir::Mutability::Mut {
                "mut ".to_string()
            } else {
                "".to_string()
            };
            let ty = local_decl.ty.to_string();
            let local_str = format!("{}{}: {}", mut_string, symbol, ty);
            if i < live_in.len() {
                live_in_str.push(local_str);
            } else {
                live_out_str.push(local_str);
            }
        }
    }
    Some(json!({
        "live_in": live_in_str,
        "live_out": live_out_str
    }))
}