use minheap::MinHeap;

pub mod file;
pub mod minheap;

const FILE_NAME: &str = "example.txt";

fn main() {
    let lines = file::get_lines(FILE_NAME);
    println!("{:?}", lines);
    let map = file::map_chars(FILE_NAME);
    let min_heap = MinHeap::from_map(&map);
    min_heap.print();
}
