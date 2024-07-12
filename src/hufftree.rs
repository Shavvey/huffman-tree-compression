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
    tree: Vec<HuffTree>,
}

impl HuffTree {
    pub fn build(min_heap: &MinHeap) -> HuffTree {
        todo!();
    }
}
