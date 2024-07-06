#![allow(dead_code)]

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
        println!("{:?}", self.arr);
        println!("Capacity: {}", self.capacity);
        println!("Size: {}", self.size);
    }
}
