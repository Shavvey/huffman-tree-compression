#![allow(dead_code)]

use crate::file;
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
    fn new(input: &str) -> Self {
        let file_input = File::create(input);
        let fi: File;
        let fo: File;
        match file_input {
            Ok(file) => fi = file,
            Err(..) => panic!("Could not open input file!"),
        };
        let file_output = file::get_output_file(input);
        match file_output {
            Ok(file) => fo = file,
            Err(..) => panic!("Could not open output file!"),
        };
        let tree = HuffTree::from_file(input);
        let map = tree.to_map();
        Serializer {
            output: fi,
            input: fo,
            code_map: map,
            huff_tree: tree,
        }
    }
}
