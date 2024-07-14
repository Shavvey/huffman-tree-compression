#![allow(dead_code)]
use crate::minheap::{MinHeap, Node};

pub struct HuffTree {
    tree: MinHeap,
}

impl HuffTree {
    // function to build the hufftree from
    // bare min_heap, we do just by inserting
    // intermediates nodes such that each node
    // now represents a leaf on the tree
    pub fn build(min_heap: &MinHeap) -> HuffTree {
        todo!();
    }

    // creates a very simple string representation of the hufftree
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        self.tree
            .heap
            .iter()
            .for_each(|node| result.push(node.item));
        result
    }

    pub fn insert(&self, node: Node) {}
}
