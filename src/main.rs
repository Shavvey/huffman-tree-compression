use minheap::MinHeap;

pub mod file;
pub mod hufftree;
pub mod minheap;

const FILE_NAME: &str = "example.txt";

fn main() {
    let min_heap = MinHeap::create_from_file(FILE_NAME);
    println!("{}", min_heap);
}
