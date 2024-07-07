#![allow(dead_code)]
use std::{collections::hash_map::HashMap, mem::swap};

#[derive(Copy, Clone, Debug)]
pub struct Node {
    item: char,
    code: u32,
    count: u32,
}

impl Node {
    pub fn print(&self) {
        println!("Character encoded: {}", self.item);
        println!("Binary code: {:b}", self.code);
        println!("Occurences: {}", self.count);
    }
}

pub struct MinHeap {
    arr: Vec<Node>, // avoding using generics for now..
    capacity: u32,
    size: u32,
}

impl MinHeap {
    // create the minheap
    pub fn new(capacity: u32) -> MinHeap {
        MinHeap {
            arr: Vec::with_capacity(capacity as usize),
            capacity,
            size: 0,
        }
    }

    pub fn from_map(map: &HashMap<char, u32>) -> MinHeap {
        let mut min_heap = MinHeap::new(map.len() as u32);
        let mut index: u32 = 0;
        for (key, val) in map {
            // build a node using the char mapping
            let node = Node {
                item: *key,
                count: *val,
                code: 0, // dont' assign a code just yet...
            };
            min_heap.arr.insert(index as usize, node);
            index += 1;
        }
        min_heap
    }

    // gets the left child in the minheap
    pub fn left_child(&self, index: u32) -> Option<&Node> {
        let l_item = 2 * index + 1;
        // check to see if we have overindexed the array
        if l_item > self.size {
            return None;
        }
        Some(&self.arr[l_item as usize])
    }

    // gets the right child in the minheap
    pub fn right_child(&self, index: u32) -> Option<&Node> {
        let r_item = 2 * index;
        // check to see if we have overindex the array
        if r_item > self.size {
            return None;
        }
        Some(&self.arr[r_item as usize])
    }

    pub fn get(&self, index: u32) -> Option<&Node> {
        if index > self.size {
            return None;
        }
        Some(&self.arr[index as usize])
    }

    pub fn print(&self) {
        for node in self.arr.iter() {
            node.print();
        }
        println!("Capacity: {}", self.capacity);
        println!("Size: {}", self.size);
    }
    pub fn swap(&mut self, i: u32, j: u32) {
        let Some(mut n1) = self.arr.get(i as usize) else { return };
        let Some(mut n2) = self.arr.get(j as usize) else { return };
        swap(&mut n1, &mut n2);
    }
}
