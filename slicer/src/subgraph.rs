use rustc_middle::ty::TyCtxt;
use rustc_middle::mir::{Body, BasicBlock, TerminatorKind};
use rustc_span::Span;
use std::collections::HashSet;

use crate::utils::*;
use crate::bb_wrapper::BasicBlocksWrapper;

pub struct SubgraphAnalysis<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    body: &'a Body<'tcx>,
    return_block: Option<BasicBlock>,
}

impl<'a, 'tcx> SubgraphAnalysis<'a, 'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, body: &'a Body<'tcx>) -> Self {
        let mut return_block: Option<BasicBlock> = None;

        for (block_index, block_data) in body.basic_blocks.iter_enumerated() {
            if let Some(terminator) = &block_data.terminator {
                if let TerminatorKind::Return = terminator.kind {
                    return_block = Some(block_index);
                }
            }
        }
        SubgraphAnalysis {
            tcx,
            body,
            return_block
        }
    }

    pub fn decompose(&self) -> HashSet<(BasicBlock, BasicBlock)> {
        let mut selected_spans: HashSet<(BasicBlock, BasicBlock)> = HashSet::new();
        let mut basic_blocks = BasicBlocksWrapper::new(&self.body.basic_blocks);
        let dominators = basic_blocks.dominators();

        if self.return_block.is_none() {
            return selected_spans;
        }
        let return_block: BasicBlock = self.return_block.unwrap();

        for (block1_index, _) in self.body.basic_blocks.iter_enumerated() {
            // block2 dominates block1
            // start -> block2 -> block1 -> return
            for block2_index in dominators.dominators(block1_index) {
                // Check if block1 post-dominates block2
                // We do this indirectly by setting the start node to block2,
                // and checking if block1 dominates the return block
                let start_node = basic_blocks.get_start_node();
                basic_blocks.set_start_node(block2_index);
                let dominators = basic_blocks.dominators();
                if dominators.is_reachable(return_block) && dominators.is_reachable(block1_index) {
                    if dominators.is_dominated_by(return_block, block1_index) {
                        selected_spans.insert((block2_index, block1_index));
                    }
                }
                basic_blocks.set_start_node(start_node);
            }
        }
        selected_spans
    }

    pub fn dominates(&self, block1: BasicBlock, block2: BasicBlock) -> bool {
        let basic_blocks = BasicBlocksWrapper::new(&self.body.basic_blocks);
        let dominators = basic_blocks.dominators();
        dominators.is_reachable(block1) && dominators.is_dominated_by(block2, block1)
    }

    pub fn post_dominates(&self, block1: BasicBlock, block2: BasicBlock) -> bool {
        let mut basic_blocks = BasicBlocksWrapper::new(&self.body.basic_blocks);
        if self.return_block.is_none() {
            return true;
        }
        let return_block: BasicBlock = self.return_block.unwrap();
        // We do this indirectly by setting the start node to block2,
        // and checking if block1 dominates the return block
        let start_node = basic_blocks.get_start_node();
        basic_blocks.set_start_node(block2);
        let dominators = basic_blocks.dominators();
        let post_dominates = dominators.is_reachable(return_block) && dominators.is_dominated_by(return_block, block1);
        basic_blocks.set_start_node(start_node);
        post_dominates
    }   
}