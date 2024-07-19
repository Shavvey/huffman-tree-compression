use std::env;

use hufftree::HuffTree;

pub mod file;
pub mod hufftree;
pub mod minheap;

const FILE_NAME: &str = "example.txt";

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let tree = HuffTree::from_file(FILE_NAME);
    let map = tree.to_map();
    println!("{:?}", map);
}
