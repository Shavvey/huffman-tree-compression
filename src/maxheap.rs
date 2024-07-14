use std::{cmp::Ordering, collections::hash_map::HashMap, fmt::Display};

use crate::file;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Node {
    pub item: char,
    pub count: u32,
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

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.item, self.count)
    }
}
impl Node {
    pub fn print(&self) {
        println!("Character encoded: {}", self.item);
        println!("Frequency: {}", self.count);
    }
}
#[derive(Clone)]
pub struct MaxHeap {
    pub heap: Vec<Node>, // heap contains all the nodes
    pub size: u32,
}

impl Display for MaxHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        self.heap
            .iter()
            // gather each node info into a string
            .for_each(|node| output.push_str(&format!("[{}, {}] ", node.item, node.count)));
        // write to the formatter
        write!(f, "{output}")
    }
}

impl MaxHeap {
    // create the minheap
    pub fn new(capacity: u32) -> Self {
        MaxHeap {
            heap: Vec::with_capacity(capacity as usize),
            size: 0,
        }
    }

    pub fn create_from_file(filename: &str) -> MaxHeap {
        let map = file::map_chars(filename);
        MaxHeap::from_map(&map)
    }

    // create a min_heap from map
    pub fn from_map(map: &HashMap<char, u32>) -> Self {
        let mut heap = MaxHeap::new(map.len() as u32);
        let mut index: usize = 0;
        for (key, val) in map {
            // build a node using the char mapping
            let node = Node {
                item: *key,
                count: *val,
            };
            heap.heap.insert(index, node);
            heap.size += 1;
            index += 1;
        }
        // call min_heapify to create the min heap property
        heap.max_heapify(0);
        // return back the heap
        heap
    }

    pub fn left(index: u32) -> u32 {
        2 * index + 1
    }

    pub fn right(index: u32) -> u32 {
        2 * index + 2
    }

    pub fn parent(index: u32) -> u32 {
        (index - 1) / 2
    }

    // gets the left child in the minheap
    pub fn left_node(&self, index: u32) -> Option<&Node> {
        let l_item = MaxHeap::left(index);
        // check to see if we have overindexed the array
        if l_item > self.size - 1 {
            return None;
        }
        Some(&self.heap[l_item as usize])
    }

    // gets the right child in the minheap
    pub fn right_node(&self, index: u32) -> Option<&Node> {
        let r_item = MaxHeap::right(index);
        // check to see if we have overindex the array
        if r_item > self.size - 1 {
            return None;
        }
        Some(&self.heap[r_item as usize])
    }

    // get the parent node
    pub fn parent_node(&self, index: u32) -> Option<&Node> {
        let p_item = (index - 1) / 2;
        // check to see if we have overindex the array
        if p_item > self.size {
            return None;
        }
        Some(&self.heap[p_item as usize])
    }

    pub fn get(&self, index: u32) -> Option<&Node> {
        if self.size == 0 {
            return None;
        }

        if index > self.size - 1 {
            return None;
        }
        Some(&self.heap[index as usize])
    }
    // method to print out all the nodes inside the heap
    pub fn print(&self) {
        for node in self.heap.iter() {
            node.print();
        }
        println!("Size: {}", self.size);
    }
    /// helper function to swap two values in the min_heap
    pub fn swap(&mut self, a: usize, b: usize) {
        // swapping using temp values
        let temp = self.heap[a];
        self.heap[a] = self.heap[b];
        self.heap[b] = temp;
    }

    // use min heapify to create min the heap property
    // where all children of a node is greater than or equal to the parent
    pub fn max_heapify(&mut self, mut idx: u32) {
        if let Some(parent) = self.get(idx) {
            let mut swap_idx = idx;
            let mut largest = parent;
            if let Some(right) = self.right_node(idx) {
                if right >= largest {
                    largest = right;
                    swap_idx = MaxHeap::right(idx);
                }
            }

            if let Some(left) = self.left_node(idx) {
                if left >= largest {
                    largest = left;
                    swap_idx = MaxHeap::left(idx);
                }
            }
            if largest.ne(&parent) {
                self.swap(idx as usize, swap_idx as usize);
                idx = swap_idx;
                self.max_heapify(idx);
            }
        }
    }
    // build a min heap using min heapify function
    pub fn build_max_heap(&mut self) {
        let n = self.size - 1;
        let idx = (n - 1) / 2;
        for i in (0..=idx).rev() {
            self.max_heapify(i);
        }
    }

    pub fn extract_max(&mut self) -> Option<Node> {
        if self.size > 0 {
            println!("{self}");
            let first = self.heap[0];
            let last = self.size - 1;
            self.heap[0] = self.heap[last as usize];
            self.heap.pop();
            self.size -= 1;
            self.max_heapify(0);
            return Some(first);
        } else {
            None
        }
    }
}
