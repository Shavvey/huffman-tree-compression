use std::env;

use hufftree::HuffTree;

pub mod file;
pub mod hufftree;
pub mod minheap;
pub mod serializer;

const FILE_NAME: &str = "example.txt";

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let tree = HuffTree::from_file(FILE_NAME);
    let map = tree.to_map();
    let _ = file::collect(FILE_NAME, &map);
    let s = tree.to_string();
    println!("{s}");
}
