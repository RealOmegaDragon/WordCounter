use std::collections::HashMap;
use std::fs;
use std::io;

fn get_file_content(file: String) -> String {

    let contents: String = fs::read_to_string(file).expect("Invalid File");

    return contents;
}

fn word_count(content: String) -> usize {

    let words: Vec<&str> = content.split_whitespace().collect();
    let length: usize = words.len();
    
    return length;
}

fn main() {
    let mut counts: HashMap<String, usize> = HashMap::new();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let files: Vec<String> = input.split(",").map(|s: &str| s.trim().to_string()).collect();

    for file in files {
        let content: String = get_file_content(file.clone());

        let count: usize = word_count(content);

        counts.insert(file, count);
    }

    for (file, count) in counts.iter() {
        println!("File: {}", file);
        println!("Word Count: {}", count);
        println!();
    }
}