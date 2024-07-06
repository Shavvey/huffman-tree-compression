use std::{char, fs::File, io::Read};
pub mod minheap;

fn main() {
    println!("Hello, world!");
}
fn read_file(file_path: &str) {
    // create file struct
    let file: File = File::create(file_path).unwrap();
    if let bytes = file.bytes() {
        for byte in bytes {
            let c: char = byte.into();
        }
    }
}
