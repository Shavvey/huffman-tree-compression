use std::fs::read_to_string;

pub fn get_lines(file_path: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}
