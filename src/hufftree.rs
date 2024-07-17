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

    pub fn print(&self) {
        self.root.print();
    }

    // function to build the hufftree from
    // bare min_heap, we do just by inserting
    // intermediates nodes such that each node
    // now represents a leaf on the tree
    pub fn build(min_heap: &mut MinHeap) -> Self {
        while min_heap.size > 1 {
            let left = min_heap.extract_min().unwrap();
            let right = min_heap.extract_min().unwrap();
            let mut node = Node::new('$', left.count + right.count);
            node.left = Subtree {
                root: Option::Some(Box::new(left)),
            };
            node.right = Subtree {
                root: Option::Some(Box::new(right)),
            };
            min_heap.insert(node);
        }
        let root_node = min_heap.extract_min().unwrap();
        Self {
            root: Subtree {
                root: Option::Some(Box::new(root_node)),
            },
        }
    }

    // creates a very simple string representation of the hufftree
    pub fn to_string(&self) -> String {
        self.root.to_string()
    }
}
