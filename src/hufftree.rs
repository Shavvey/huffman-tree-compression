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
        while min_heap.size > 1 {
            let left = min_heap.extract_max();
            let right = min_heap.extract_max();
            let node = Node {
                count: HuffTree::sum(left, right),
                item: '$',
            };
            // insert intermediate node
            ht.insert(node);
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
    pub fn sum(l: Option<Node>, r: Option<Node>) -> u32 {
        let cl: u32;
        let cr: u32;
        match l {
            None => cl = 0,
            Some(left) => cl = left.count,
        }
        match r {
            None => cr = 0,
            Some(right) => cr = right.count,
        }
        cr + cl
    }
}
