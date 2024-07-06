use crate::minheap::MinHeap;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn get_lines(file_path: &str) -> Vec<String> {
    // collect results into a vector of strings
    let mut result: Vec<String> = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        result.push(line.to_string());
    }
    // return the result
    result
}

pub fn get_heap(file_path: &str) -> MinHeap {
    let mut min_heap = MinHeap::new(5);
    let mut map: HashMap<char, u32> = HashMap::new();
    let lines = get_lines(file_path);
    for line in lines {
        for char in line.chars() {
            match &map.get(&char) {
                Some(_) => todo!(),
                None => todo!(),
            }
        }
    }
    min_heap
}
