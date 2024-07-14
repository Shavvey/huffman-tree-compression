use std::env;

use hufftree::HuffTree;
use maxheap::MaxHeap;

pub mod file;
pub mod hufftree;
pub mod maxheap;

const FILE_NAME: &str = "example.txt";

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut min_heap = MaxHeap::create_from_file(FILE_NAME);
    let tree = HuffTree::build(&mut min_heap);
}
