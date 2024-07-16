#![allow(dead_code)]

use crate::minheap::{MinHeap, Node};

pub struct HuffNode {
    value: Node,
    left: Subtree,
    right: Subtree,
}
impl HuffNode {
    // instantiate new node, creating new references to left and right subtree, but keep them empty
    pub fn new(value: Node) -> Self {
        Self {
            value,
            left: Subtree::new(),
            right: Subtree::new(),
        }
    }
}
// either contains a value, a reference to another, or nothing
pub struct Subtree(Option<Box<HuffNode>>);

impl Subtree {
    pub fn new() -> Self {
        Self(None)
    }
    pub fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some(n) => 1 + n.left.len() + n.right.len(),
        }
    }
}

pub struct HuffTree {
    root: Subtree, // we can describe the tree as just a collection of subtrees
}

impl HuffTree {
    pub fn new() -> Self {
        Self {
            root: Subtree::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.root.len()
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
