use std::fs;

pub fn load(file: &str) -> String {
    let contents =
        fs::read_to_string(file).expect("Something went wrong reading the file");
    return contents;
}