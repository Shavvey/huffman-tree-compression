#![allow(dead_code)]

struct MinHeap {
    arr: Vec<u32>, // avoding using generics for now..
    capacity: u32,
    size: u32,
}

impl MinHeap {
    pub fn new(capacity: u32) -> MinHeap {
        MinHeap {
            arr: Vec::with_capacity(capacity as usize),
            capacity,
            size: 0,
        }
    }

    pub fn left(&self, index: u32) -> Option<u32> {
        let l_item = 2 * index + 1;
        if l_item > self.size {
            return None;
        }
        Some(self.arr[l_item as usize])
    }
    pub fn right(&self, index: u32) -> Option<u32> {
        let r_item = 2 * index;
        if r_item > self.size {
            return None;
        }
        Some(self.arr[r_item as usize])
    }
}
