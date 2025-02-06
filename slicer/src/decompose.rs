use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

use serde_json::json;

use crate::utils::*;
use crate::SlicerConfig;

use crate::visitor;
use crate::block_visitor::BlockVisitor;

use petgraph::visit::DfsPostOrder;

pub fn process_span<'tcx>(tcx: &TyCtxt<'_>,
                          span: Span,
                          chunk_list: &Vec<(usize, Span)>)
                -> (String, Vec<Span>, Vec<(usize, Span)>) {

    let mut chunk_list = chunk_list.clone();
    // First, sort chunk_list in decreasing order of length
    chunk_list.sort_by(|(_, span1), (_, span2)| {
        get_span_bytes(&span2).cmp(&get_span_bytes(&span1))
    });

    let mut sub_chunks: Vec<(usize, Span)> = Vec::new();

    for &(i, this_span) in chunk_list.iter() {
        if span.contains(this_span) && ! span.source_equal(this_span) {
            // Check if this_span is contained in any of the sub_chunks so far
            if sub_chunks.iter().any(|(_, sub_span)| sub_span.contains(this_span)) {
                continue;
            }
            sub_chunks.push((i, this_span));
        }
    }

    // First sort the sub_chunks in increasing order of start position
    sub_chunks.sort_by(|(_, span1), (_, span2)| {
        span1.lo().cmp(&span2.lo())
    });
    if sub_chunks.len() == 0 {
        return (span_to_string(*tcx, &span), vec![span], sub_chunks);
    }
    let mut snippet = String::new();
    let mut snippet_pieces: Vec<Span> = vec![];
    let (first_ind, first_sub_span) = sub_chunks.first().unwrap();
    let first_piece = span_start_to_start(span, *first_sub_span);
    snippet.push_str(&span_to_string(*tcx, &first_piece));
    snippet_pieces.push(first_piece);
    // Append <chunk `first_ind`> to the snippet
    snippet.push_str(&format!("<<chunk {}>>", first_ind));
    let mut prev_span = *first_sub_span;

    for &(i, sub_span) in sub_chunks.iter().skip(1) {
        let this_piece = span_end_to_start(prev_span, sub_span);
        snippet_pieces.push(this_piece);
        snippet.push_str(&span_to_string(*tcx, &this_piece));
        snippet.push_str(&format!("<<chunk {}>>", i));
        prev_span = sub_span;
    }

    let last_piece = span_end_to_end(prev_span, span);
    snippet.push_str(&span_to_string(*tcx, &last_piece));
    snippet_pieces.push(last_piece);

    (snippet, snippet_pieces, sub_chunks)
}

pub fn decompose(tcx: TyCtxt, config: SlicerConfig) -> Vec<serde_json::Value> {
    
    let mut visitor = visitor::CallgraphVisitor::new(&tcx);
    tcx.hir().visit_all_item_likes_in_crate(&mut visitor);

    let mut dfs_order: Vec<DefId> = vec![];

    for (func_def_id, _) in visitor.functions.iter() {
        if dfs_order.contains(func_def_id) {
            continue;
        }
        let mut dfs = DfsPostOrder::new(&visitor.graph, *func_def_id);
        // Collect the results of the DFS traversal
        while let Some(this_def_id) = dfs.next(&visitor.graph) {
            if dfs_order.contains(&this_def_id) {
                continue;
            }
            dfs_order.push(this_def_id);
        }
    }

    let mut json_output: Vec<serde_json::Value> = vec![];

    for func_def_id in dfs_order.iter() {
        // Check if it's a local function
        if !visitor.functions.contains_key(&func_def_id) {
            continue;
        }
        let func_span = visitor.functions.get(&func_def_id).unwrap();
        let func_item = tcx.hir().expect_item(func_def_id.expect_local());
        let rustc_hir::ItemKind::Fn(_, _, body_id) = &func_item.kind else {
            panic!("Expected a function definition, found {:?}", func_item.kind);
        };
        let body_defid = tcx.hir().body_owner_def_id(*body_id).to_def_id();
        let mir_body = get_mir_fn_from_defid(&tcx, body_defid).unwrap();

        let num_lines = get_span_lines(tcx, &mir_body.span);
        
        let mut this_func = json!({
            "func_defid": format!("{:?}", func_def_id),
            "span": format!("{:?}", func_span),
            "pieces": [format!("{:?}", func_span)],
            "sub_chunks": [],
            "num_lines": num_lines,
            "source": span_to_string(tcx, func_span),
            "calls": [],
            "globals": [],
            "imports": [],
            "chunks": []
        });

        if num_lines > config.max_lines {
            let mut block_visitor = BlockVisitor::new(&tcx, func_def_id.expect_local(), config.clone());
            tcx.hir().visit_all_item_likes_in_crate(&mut block_visitor);
            let aggregated_spans = block_visitor.compressed_spans
                                                                .iter()
                                                                .map(|(start, end)|
                                                                    start.span().to(*end.span()))
                                                                .enumerate()
                                                                .collect::<Vec<(usize, _)>>();

            let (main_func_source,
                 main_func_pieces,
                 main_sub_chunks) = process_span(&tcx, *func_span, &aggregated_spans);

            this_func["source"] = json!(main_func_source); // Overwrite
            this_func["num_lines"] = json!(main_func_pieces.iter().map(|span| get_span_lines(tcx, span)).sum::<usize>()); // Overwrite
            this_func["pieces"] = json!(main_func_pieces.iter().map(|span| format!("{:?}", span)).collect::<Vec<String>>());
            this_func["sub_chunks"] = json!(main_sub_chunks.iter().map(|(i, _)| *i).collect::<Vec<usize>>());

            for (chunk_id, current_span) in aggregated_spans.iter() {
                let (snippet, snippet_pieces, sub_chunks) = process_span(&tcx, *current_span, &aggregated_spans);
                let chunk = json!({
                    "chunk_id": chunk_id,
                    "span": format!("{:?}", current_span),
                    "pieces": snippet_pieces.iter().map(|span| format!("{:?}", span)).collect::<Vec<String>>(),
                    "num_lines": snippet_pieces.iter().map(|span| get_span_lines(tcx, span)).sum::<usize>(),
                    "source": snippet,
                    "sub_chunks": sub_chunks.iter().map(|(i, _)| *i).collect::<Vec<usize>>()
                });
                this_func["chunks"].as_array_mut().unwrap().push(chunk);
            }
        }
        json_output.push(this_func);
    }
    json_output
}

pub fn extract_func_deps(tcx: TyCtxt, func: &mut serde_json::Value) -> Option<serde_json::Value> {
    
    let mut returned_json = json!({
        "calls": [],
        "globals": [],
        "imports": []
    });

    let mut visitor = visitor::CallgraphVisitor::new(&tcx);
    tcx.hir().visit_all_item_likes_in_crate(&mut visitor);

    let func_defid = get_defid_from_defid_string(tcx, func["func_defid"].as_str().unwrap().to_string());
    if func_defid.is_none() {
        return None;
    }
    let func_defid = func_defid.unwrap();
    let span = get_span_from_span_str(tcx, func["span"].as_str().unwrap().to_string());

    // Get all the imports in the same file as the function definition
    let imports = get_imports_from_same_file(&span, &visitor.imports, &tcx);
    if imports.len() > 0 {
        for import in imports {
            let import_span = import;
            let import_json = json!({
                "span": format!("{:?}", import_span),
                "source": span_to_string(tcx, &import_span)
            });
            returned_json["imports"].as_array_mut().unwrap().push(import_json);
        }
    }
    // Get all the calls to this function
    for call in visitor.static_calls.iter().chain(visitor.dynamic_calls.iter()) {
        if call.callee == func_defid {
            let call_span = get_parent_span(&call.call_expr, &tcx).unwrap();
            let call_json = json!({
                "caller": if call.caller.is_some() { format!("{:?}", call.caller.unwrap()) } else { "".to_string()},
                "span": format!("{:?}", call_span),
                "source": span_to_string(tcx, &call_span)
            });
            returned_json["calls"].as_array_mut().unwrap().push(call_json);
        }
    }
    // Get globals corresponding to this function
    if let Some(globals) = visitor.globals.get(&func_defid) {
        if globals.len() > 0 {
            for global in globals {
                let global_span = global;
                let global_json = json!({
                    "span": format!("{:?}", global_span),
                    "source": span_to_string(tcx, &global_span)
                });
                returned_json["globals"].as_array_mut().unwrap().push(global_json);
            }
        }
    }
    Some(returned_json)
}