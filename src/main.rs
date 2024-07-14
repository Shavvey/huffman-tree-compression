use hufftree::HuffTree;
use minheap::MinHeap;

pub mod file;
pub mod hufftree;
pub mod minheap;

const FILE_NAME: &str = "example.txt";

fn main() {
    let mut min_heap = MinHeap::create_from_file(FILE_NAME);
    println!("{min_heap}");
    let tree = HuffTree::build(&mut min_heap);
    let str = tree.to_string();
    tree.tree.print();
    println!("{str}");
}
