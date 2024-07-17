#![allow(dead_code)]

use crate::minheap::{MinHeap, Node, Subtree};

pub struct HuffTree {
    root: Subtree, // we can describe the tree as just a collection of subtrees
}

impl HuffTree {
    pub fn new() -> Self {
        Self {
            root: Subtree::new(),
        }
    }

    // return len of entire tree, uses recursive call of subtree len() method
    pub fn len(&self) -> usize {
        self.root.len()
    }

    // function to build the hufftree from
    // bare min_heap, we do just by inserting
    // intermediates nodes such that each node
    // now represents a leaf on the tree
    pub fn build(min_heap: &mut MinHeap) -> Self {
        todo!();
    }

    // creates a very simple string representation of the hufftree
    pub fn to_string(&self) -> String {
        todo!();
    }
}
