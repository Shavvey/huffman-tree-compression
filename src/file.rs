use std::collections::HashMap;
use std::fs::{read_to_string, File};

// get the lines of the textfile, store them into the a vector of strings
pub fn get_lines(file_path: &str) -> Vec<String> {
    // collect results into a vector of strings
    let mut result: Vec<String> = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        result.push(line.to_string());
    }
    // return the result
    result
}

// create a mapping of chars with their character occurences in the file path
pub fn map_chars(file_path: &str) -> HashMap<char, u32> {
    // create a mutable hashmap that will associated a char, as a key, to the occruences inside
    // textfile, its value
    let mut map: HashMap<char, u32> = HashMap::new();
    // using lines to in textfile to count char occurences
    let lines = get_lines(file_path);
    for line in lines {
        for char in line.chars() {
            match map.get_mut(&char) {
                Some(val) => {
                    *val += 1;
                }
                None => {
                    map.insert(char, 1);
                }
            }
        }
    }
    map
}

pub fn write_output(file_name: &str, code_map: &HashMap<char, String>) {
    // first get the lines from the textfile, using file_name
    let lines = get_lines(file_name);
    let fs = file_name.replace(".txt", ".bin");
    println!("New filename: {}", fs);
    let file = File::create_new("./".to_owned() + &fs);
    for line in lines {
        for char in line.chars() {
            let bit_str = code_map.get(&char);
            if let Some(str) = &bit_str {
                println!("{char} : {str}");
            } else {
                println!("Found no key!");
            }
        }
    }
}
