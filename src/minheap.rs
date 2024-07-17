use crate::file;
use std::{cmp::Ordering, collections::hash_map::HashMap, fmt::Display};

pub struct Subtree {
    root: Option<Box<Node>>,
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

pub struct Node {
    item: char,
    count: u32,
    left: Subtree,
    right: Subtree,
}

impl Node {
    pub fn new(item: char, count: u32) -> Self {
        Self {
            item,
            count,
            left: Subtree::new(),
            right: Subtree::new(),
        }
    }
    pub fn count(&self) -> u32 {
        self.count
    }
    pub fn item(&self) -> char {
        self.item
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        let count = self.count.eq(&other.count);
        let char = self.item.eq(&other.item);
        if count && char {
            return true;
        } else {
            false
        }
    }
}
impl Eq for Node {}

// implementing a very simple order and partial ordering scheme for the minheap nodes
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.item.partial_cmp(&other.item)
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
pub struct MinHeap {
    heap: Vec<Node>, // heap contains all the nodes
    size: u32,
}

impl Display for MinHeap {
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

impl MinHeap {
    // create the minheap
    pub fn new(capacity: u32) -> Self {
        Self {
            heap: Vec::with_capacity(capacity as usize),
            size: 0,
        }
    }

    pub fn create_from_file(filename: &str) -> MinHeap {
        let map = file::map_chars(filename);
        Self::from_map(&map)
    }

    // create a min_heap from map
    pub fn from_map(map: &HashMap<char, u32>) -> Self {
        let mut heap = MinHeap::new(map.len() as u32);
        let mut index: usize = 0;
        for (key, val) in map {
            // build a node using the char mapping
            let node = Node {
                item: *key,
                count: *val,
                left: Subtree::new(),
                right: Subtree::new(),
            };
            heap.heap.insert(index, node);
            heap.size += 1;
            index += 1;
        }
        // call min_heapify to create the min heap property
        heap.min_heapify(0);
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
    pub fn size(&self) -> u32 {
        self.size
    }

    // gets the left child in the minheap
    pub fn left_node(&self, index: u32) -> Option<&Node> {
        let l_item = MinHeap::left(index);
        // check to see if we have overindexed the array
        if l_item > self.size - 1 {
            return None;
        }
        Some(&self.heap[l_item as usize])
    }

    // gets the right child in the minheap
    pub fn right_node(&self, index: u32) -> Option<&Node> {
        let r_item = MinHeap::right(index);
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

    pub fn insert(&mut self, node: Node) {
        // start at very end of tree
        self.heap.push(node);
        self.size += 1;
        let mut i = self.size - 1;
        while (i != 0) && (node.count > self.parent_node(i).unwrap().count) {
            let curr = i as usize;
            let parent = MinHeap::parent(i) as usize;
            self.swap(curr, parent);
            i = MinHeap::parent(i);
        }
    }
    // use min heapify to create min the heap property
    // where all children of a node is greater than or equal to the parent
    pub fn min_heapify(&mut self, mut idx: u32) {
        if let Some(parent) = self.get(idx) {
            let mut swap_idx = idx;
            let mut smallest = parent;
            if let Some(right) = self.right_node(idx) {
                if right < smallest {
                    smallest = right;
                    swap_idx = MinHeap::right(idx);
                }
            }

            if let Some(left) = self.left_node(idx) {
                if left < smallest {
                    smallest = left;
                    swap_idx = MinHeap::left(idx);
                }
            }
            if smallest.ne(&parent) {
                self.swap(idx as usize, swap_idx as usize);
                idx = swap_idx;
                self.min_heapify(idx);
            }
        }
    }
    // build a min heap using min heapify function
    pub fn build_max_heap(&mut self) {
        let n = self.size - 1;
        let idx = (n - 1) / 2;
        for i in (0..=idx).rev() {
            self.min_heapify(i);
        }
    }

    pub fn extract_min(&mut self) -> Option<Node> {
        if self.size > 0 {
            println!("{self}");
            let first = self.heap[0];
            let last = self.size - 1;
            self.heap[0] = self.heap[last as usize];
            self.heap.pop();
            self.size -= 1;
            self.min_heapify(0);
            return Some(first);
        } else {
            None
        }
    }
}
