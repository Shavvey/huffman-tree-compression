pub mod file;
pub mod minheap;

const FILE_NAME: &str = "example.txt";

fn main() {
    println!("Hello, world!");
    let result = file::get_lines(FILE_NAME);
    println!("{:?}", result);
}
