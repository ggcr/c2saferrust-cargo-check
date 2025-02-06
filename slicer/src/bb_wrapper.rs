use rustc_data_structures::graph;
use rustc_middle::mir::{BasicBlocks, BasicBlock, START_BLOCK};
use crate::dominators::{dominators, Dominators};

// All this is copied from the original BasicBlocks implementation for PredecessorCache
use rustc_data_structures::sync::OnceCell;
use rustc_index::vec::IndexVec;
use smallvec::SmallVec;

// Typically 95%+ of basic blocks have 4 or fewer predecessors.
pub type MySuccessors = IndexVec<BasicBlock, SmallVec<[BasicBlock; 4]>>;

#[derive(Clone, Debug)]
pub(super) struct SuccessorCache {
    cache: OnceCell<MySuccessors>,
}

impl SuccessorCache {
    #[inline]
    pub(super) fn new() -> Self {
        SuccessorCache { cache: OnceCell::new() }
    }

    /// Returns the successor graph for this MIR.
    #[inline]
    pub(super) fn compute(
        &self,
        basic_blocks: &BasicBlocks,
    ) -> &MySuccessors {
        self.cache.get_or_init(|| {
            let mut preds = IndexVec::from_elem(SmallVec::new(), basic_blocks);
            for (bb, data) in basic_blocks.iter_enumerated() {
                if let Some(term) = &data.terminator {
                    for succ in term.successors() {
                        preds[bb].push(succ);
                    }
                }
            }
            preds
        })
    }
}


#[derive(Clone, Debug)]
pub struct BasicBlocksWrapper<'wrapper, 'tcx>{
    inner: &'wrapper BasicBlocks<'tcx>,
    start_node: BasicBlock,
    reverse: bool,
    successor_cache: SuccessorCache,
}

impl<'wrapper, 'tcx> BasicBlocksWrapper<'wrapper, 'tcx> {

    pub fn new(basic_blocks: &'wrapper BasicBlocks<'tcx>) -> Self {
        BasicBlocksWrapper {
            inner: basic_blocks,
            start_node: START_BLOCK,
            reverse: false,
            successor_cache: SuccessorCache::new(),
        }
    }
    
    pub fn dominators(&self) -> Dominators<BasicBlock> {
        dominators(&self)
    }

    pub fn set_start_node(&mut self, start_node: BasicBlock) {
        self.start_node = start_node;
    }

    pub fn get_start_node(&mut self) -> BasicBlock {
        self.start_node
    }

    pub fn reverse_direction(&mut self) {
        self.reverse = !self.reverse;
    }

    /// Returns successors for each basic block.
    #[inline]
    pub fn successors(&self) -> &MySuccessors {
        self.successor_cache.compute(&self.inner)
    }
}

impl<'wrapper, 'tcx> graph::DirectedGraph for BasicBlocksWrapper<'wrapper, 'tcx> {
    type Node = BasicBlock;
}

impl<'wrapper, 'tcx> graph::WithNumNodes for BasicBlocksWrapper<'wrapper, 'tcx> {
    fn num_nodes(&self) -> usize {
        self.inner.num_nodes()
    }
}

// Override
impl<'wrapper, 'tcx> graph::WithStartNode for BasicBlocksWrapper<'wrapper, 'tcx> {
    fn start_node(&self) -> Self::Node {
        self.start_node
    }
}

// Override
impl<'wrapper, 'tcx> graph::WithSuccessors for BasicBlocksWrapper<'wrapper, 'tcx> {
    fn successors(&self, node: BasicBlock) -> <Self as graph::GraphSuccessors<'_>>::Iter {
        if self.reverse {
            self.inner.predecessors()[node].iter().copied()
        } else {
            self.successors()[node].iter().copied()
        }
    }
}

// Override
impl<'wrapper, 'tcx> graph::WithPredecessors for BasicBlocksWrapper<'wrapper, 'tcx> {
    fn predecessors(&self, node: BasicBlock) -> <Self as graph::GraphPredecessors<'_>>::Iter {
        if self.reverse {
            self.successors()[node].iter().copied()
        } else {
            self.inner.predecessors()[node].iter().copied()
        }
    }
}
// Override
impl<'tcx, 'graph, 'wrapper> graph::GraphSuccessors<'graph> for BasicBlocksWrapper<'wrapper, 'tcx> {
    type Item = BasicBlock;
    type Iter = std::iter::Copied<std::slice::Iter<'graph, BasicBlock>>;
}

impl<'tcx, 'graph, 'wrapper> graph::GraphPredecessors<'graph> for BasicBlocksWrapper<'wrapper, 'tcx> {
    type Item = BasicBlock;
    type Iter = std::iter::Copied<std::slice::Iter<'graph, BasicBlock>>;
}