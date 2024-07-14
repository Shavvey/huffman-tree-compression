#![allow(dead_code)]

use crate::maxheap::{MaxHeap, Node};

pub struct HuffTree {
    pub tree: MaxHeap,
}

impl HuffTree {
    pub fn new(capacity: u32) -> HuffTree {
        HuffTree {
            tree: MaxHeap::new(capacity),
        }
    }
    // function to build the hufftree from
    // bare min_heap, we do just by inserting
    // intermediates nodes such that each node
    // now represents a leaf on the tree
    pub fn build(min_heap: &mut MaxHeap) -> HuffTree {
        let mut ht = HuffTree::new(min_heap.size);
        ht.tree = min_heap.clone();
        for _ in 0..min_heap.size / 2 {
            let left = min_heap.extract_max();
            let right = min_heap.extract_max();
            let par = Node {
                count: left.count + right.count,
                item: '$',
            };
            // insert intermediate node
            ht.insert(par);
        }
        ht.tree.print();
        ht
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

    pub fn insert(&mut self, node: Node) {
        // start at very end of tree
        self.tree.size += 1;
        self.tree.heap.push(node);
        let mut i = self.tree.size - 1;
        while (i != 0) && (node.count > self.tree.parent_node(i).unwrap().count) {
            let curr = i as usize;
            let parent = MaxHeap::parent(i) as usize;
            self.tree.swap(curr, parent);
            i = MaxHeap::parent(i);
        }
    }
}
