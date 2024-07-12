#![allow(dead_code)]
use crate::minheap::MinHeap;

pub struct HuffNode {
    freq: u32,
    item: char,
}

impl HuffNode {
    pub fn new(freq: u32, item: char) -> HuffNode {
        HuffNode { freq, item }
    }
}

pub struct HuffTree {
    capacity: u32,
    size: u32,
    tree: Vec<HuffNode>,
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
        self.tree.iter().for_each(|node| result.push(node.item));
        result
    }
}
