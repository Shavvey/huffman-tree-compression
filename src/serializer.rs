use crate::HuffTree;
use std::collections::HashMap;
use std::fs::File;

pub struct Serializer {
    output: File,
    input: File,
    code_map: HashMap<char, String>,
    huff_tree: HuffTree,
}

impl Serializer {
    fn new(input_file: &str) -> Self {
        todo!();
    }
}
