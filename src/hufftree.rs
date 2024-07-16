#![allow(dead_code)]

use std::borrow::Borrow;

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

    pub fn to_huff_node(node: &Node) -> Self {
        Self {
            value: *node,
            left: Subtree::new(),
            right: Subtree::new(),
        }
    }
}
// either contains a value, a reference to another node, or nothing
pub struct Subtree {
    root: Option<Box<HuffNode>>,
}

impl Subtree {
    pub fn new() -> Self {
        Self { root: None }
    }
    pub fn len(&self) -> usize {
        match &self.root {
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

    // return len of entire tree, uses recursive call of subtree len() method
    pub fn len(&self) -> usize {
        self.root.len()
    }

    // function to build the hufftree from
    // bare min_heap, we do just by inserting
    // intermediates nodes such that each node
    // now represents a leaf on the tree
    pub fn build(min_heap: &mut MinHeap) -> Self {
        while min_heap.size() > 1 {
            let left = min_heap.extract_min().unwrap();
            let right = min_heap.extract_min().unwrap();
            let node = Node::new('$', left.count() + right.count());
            min_heap.insert(node);
            let left_huff = HuffNode::to_huff_node(&left);
            let right_huff = HuffNode::to_huff_node(&right);
            let mut huff_node = HuffNode::new(node);
            huff_node.left = Subtree {
                root: Option::Some(Box::new(left_huff)),
            };
            huff_node.right = Subtree {
                root: Option::Some(Box::new(right_huff)),
            };
            huff_node.value = node;
        }
        let root_node = min_heap.extract_min().unwrap();
        let huff_root = HuffNode::to_huff_node(&root_node);
        Self {
            root: Subtree {
                root: Option::Some(Box::new(huff_root)),
            },
        }
    }

    // creates a very simple string representation of the hufftree
    pub fn to_string(&self) -> String {
        todo!();
    }
}
