use minheap::MinHeap;

pub mod file;
pub mod minheap;

const FILE_NAME: &str = "example.txt";

fn main() {
    let lines = file::get_lines(FILE_NAME);
    println!("{:?}", lines);
    let map = file::map_chars(FILE_NAME);
    let mut min_heap = MinHeap::from_map(&map);
    println!("===HEAP===");
    min_heap.print();
    println!("===MIN HEAP===");
    min_heap.build_min_heap();
    min_heap.print();
}
