use std::collections::HashMap;
use std::fs::read_to_string;

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
    let mut map: HashMap<char, u32> = HashMap::new();
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
