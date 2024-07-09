#![allow(dead_code)]
use std::{cmp::Ordering, collections::hash_map::HashMap, mem::swap};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Node {
    item: char,
    code: u32,
    count: u32,
}
// implementing a very simple order and partial ordering scheme for the minheap nodes
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.count.partial_cmp(&other.count)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl Node {
    pub fn print(&self) {
        println!("Character encoded: {}", self.item);
        println!("Binary code: {:b}", self.code);
        println!("Occurences: {}", self.count);
    }
}

pub struct MinHeap {
    heap: Vec<Node>, // heap contains all the nodes
    capacity: u32,   // capacity of the given minheap
    size: u32,
}

impl MinHeap {
    // create the minheap
    pub fn new(capacity: u32) -> Self {
        MinHeap {
            heap: Vec::with_capacity(capacity as usize),
            capacity,
            size: 0,
        }
    }

    pub fn from_map(map: &HashMap<char, u32>) -> Self {
        let mut min_heap = MinHeap::new(map.len() as u32);
        let mut index: u32 = 0;
        for (key, val) in map {
            // build a node using the char mapping
            let node = Node {
                item: *key,
                count: *val,
                code: 0, // dont' assign a code just yet...
            };
            min_heap.heap.insert(index as usize, node);
            min_heap.size += 1;
            index += 1;
        }
        min_heap.capacity = 0;
        min_heap
    }

    // gets the left child in the minheap
    pub fn left_child(&self, index: u32) -> Option<&Node> {
        let l_item = 2 * index + 1;
        // check to see if we have overindexed the array
        if l_item > self.size {
            return None;
        }
        Some(&self.heap[l_item as usize])
    }

    // gets the right child in the minheap
    pub fn right_child(&self, index: u32) -> Option<&Node> {
        let r_item = 2 * index;
        // check to see if we have overindex the array
        if r_item > self.size {
            return None;
        }
        Some(&self.heap[r_item as usize])
    }

    // get the parent node
    pub fn parent(&self, index: u32) -> Option<&Node> {
        let p_item = (index - 1) / 2;
        // check to see if we have overindex the array
        if p_item > self.size {
            return None;
        }
        Some(&self.heap[p_item as usize])
    }

    pub fn get(&self, index: u32) -> Option<&Node> {
        if index > self.size {
            return None;
        }
        Some(&self.heap[index as usize])
    }
    // method to print out all the nodes inside the heap
    pub fn print(&self) {
        for node in self.heap.iter() {
            node.print();
        }
        println!("Capacity: {}", self.capacity);
        println!("Size: {}", self.size);
    }

    // swap two elements at index j and i
    // this needs to be used during the minheapify algorithm
    pub fn swap(&mut self, i: u32, j: u32) {
        let Some(mut n1) = self.heap.get(i as usize) else { return };
        let Some(mut n2) = self.heap.get(j as usize) else { return };
        swap(&mut n1, &mut n2);
    }
    // use min heapify to create min the heap property
    // where all children of a node is greater than or equal to the parent
    pub fn min_heapify(&mut self) {}
}
