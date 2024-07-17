use std::env;

use hufftree::HuffTree;
use minheap::MinHeap;

pub mod file;
pub mod hufftree;
pub mod minheap;

const FILE_NAME: &str = "example.txt";

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut min_heap = MinHeap::create_from_file(FILE_NAME);
    println!("{}", min_heap);
}
