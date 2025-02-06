use rustc_hir::{intravisit, def_id::LocalDefId, Block, Stmt, Expr};
use rustc_middle::ty::TyCtxt;
use rustc_middle::hir::nested_filter;
use rustc_span::Span;

use crate::utils::*;
use crate::SlicerConfig;

macro_rules! skip_generated_code {
    ($span: expr) => {
        if $span.in_derive_expansion()
        // $span.from_expansion() || $span.is_dummy()
        {
            return;
        }
    };
}

#[derive(Copy, Clone)]
pub enum StmtOrExpr<'tcx> {
    Stmt(&'tcx Stmt<'tcx>),
    Expr(&'tcx Expr<'tcx>),
}

impl StmtOrExpr<'_> {
    pub fn span(&self) -> &Span {
        match self {
            StmtOrExpr::Stmt(stmt) => &stmt.span,
            StmtOrExpr::Expr(expr) => &expr.span,
        }
    }
}

pub struct BlockVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    func: LocalDefId,
    pub compressed_spans: Vec<(StmtOrExpr<'tcx>, StmtOrExpr<'tcx>)>,
    config: SlicerConfig,
}

impl<'tcx> BlockVisitor<'tcx> {
    pub fn new(tcx: &TyCtxt<'tcx>, func: LocalDefId, config: SlicerConfig) -> BlockVisitor<'tcx> {
        BlockVisitor{
            tcx: *tcx,
            func,
            compressed_spans: Vec::new(),
            config
        }
    }

    pub fn calc_length<'a>(&self, aggregated_span: Span) -> usize {
        let mut num_lines = get_span_lines(self.tcx, &aggregated_span);
        // Now subtract the lengths of all the spans in self.compressed_spans
        // that overlap with aggregated_span
        for (start, end) in self.compressed_spans.iter() {
            let this_span = start.span().to(*end.span());
            if aggregated_span.contains(this_span) && ! aggregated_span.source_equal(this_span) {
                num_lines -= self.calc_length(this_span);
            }
        }
        num_lines
    }
}

impl<'tcx> intravisit::Visitor<'tcx> for BlockVisitor<'tcx> {

    type NestedFilter = nested_filter::OnlyBodies;

    fn nested_visit_map(&mut self) -> Self::Map {
        self.tcx.hir()
    }
    
    fn visit_item(&mut self, item: &'tcx rustc_hir::Item) {
        skip_generated_code!(item.span);
        // Visit the item
        if let rustc_hir::ItemKind::Fn(_, _, _) = item.kind {
            if item.def_id == self.func {
                // Only continue inside the item if it is the function we are looking for
                intravisit::walk_item(self, item)
            }
        }
    }

    fn visit_expr(&mut self, expr: &'tcx Expr) {
        // println!("Reached an expr with span {:?}", expr.span);
        skip_generated_code!(expr.span);
        // println!("Walking it");
        intravisit::walk_expr(self, expr);
    }

    fn visit_stmt(&mut self, stmt: &'tcx Stmt) {
        skip_generated_code!(stmt.span);
        intravisit::walk_stmt(self, stmt);
    }

    fn visit_block(&mut self, block: &'tcx Block) {

        skip_generated_code!(block.span);
        
        let all_stmts_exprs = block.stmts.iter().map(|stmt| StmtOrExpr::Stmt(stmt))
        .chain(block.expr.iter().map(|expr| StmtOrExpr::Expr(expr))).collect::<Vec<_>>();

        for stmt in all_stmts_exprs.iter() {
            let stmt_length = self.calc_length(*stmt.span());
            if stmt_length > self.config.max_lines {
                match stmt {
                    StmtOrExpr::Stmt(stmt) => {
                        intravisit::walk_stmt(self, stmt);
                    }
                    StmtOrExpr::Expr(expr) => {
                        intravisit::walk_expr(self, expr);
                    }
                }
            }
            // This is what we would ideally like, but it is not guaranteed
            // assert!(self.calc_length(*stmt.span()) <= self.config.max_lines);
        }

        let mut i: usize = 0;
        let mut j: usize = 0;

        while j < all_stmts_exprs.len() {
            if self.calc_length((all_stmts_exprs[i].span().to(*all_stmts_exprs[j].span()))) > self.config.max_lines {
                if j == i {
                    // This should ideally have been taken care of in the previous loop
                    // But just in case
                    self.compressed_spans.push((all_stmts_exprs[i], all_stmts_exprs[i]));
                    i += 1;
                    j += 1;
                    continue;
                }
                // Adding the j-th statement caused the length to spill over self.config.max_lines lines
                // So take i..j and form a piece from it
                let block_length = self.calc_length((all_stmts_exprs[i].span().to(*all_stmts_exprs[j-1].span())));
                if block_length >= self.config.min_lines {
                    self.compressed_spans.push((all_stmts_exprs[i], all_stmts_exprs[j-1]));
                    i = j;
                } else {
                    i += 1;
                    j = i;
                }
            }
            else {
                j += 1;
            }
        }
        // We have some left overs
        if i < all_stmts_exprs.len() {
            let block_length = self.calc_length((all_stmts_exprs[i].span().to(*all_stmts_exprs.last().unwrap().span())));
            if block_length >= self.config.min_lines {
                self.compressed_spans.push((all_stmts_exprs[i], *all_stmts_exprs.last().unwrap()));
            }
        }
    }
}