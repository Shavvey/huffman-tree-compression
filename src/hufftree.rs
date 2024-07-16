#![allow(dead_code)]

use crate::minheap::{MinHeap, Node};
// either contains a value, a reference to another, or nothing
pub struct Subtree(Option<Box<Node>>);

pub struct HuffTree {
    root: Subtree,
}

impl HuffTree {
    pub fn new(capacity: u32) -> HuffTree {
        todo!();
    }
    // function to build the hufftree from
    // bare min_heap, we do just by inserting
    // intermediates nodes such that each node
    // now represents a leaf on the tree
    pub fn build(min_heap: &mut MinHeap) -> HuffTree {
        todo!();
    }

    // creates a very simple string representation of the hufftree
    pub fn to_string(&self) -> String {
        todo!();
    }
}
