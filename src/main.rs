use std::collections::HashMap;
use std::fs;
use std::io;
use std::thread;

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
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let files: Vec<String> = input.split(",").map(|s: &str| s.trim().to_string()).collect();

    let mut handles = Vec::new();

    for file in files {
        let handle = thread::spawn(move || {
            let content: String = get_file_content(file.clone());
            let count: usize = word_count(content);
            return (file, count);
        });
        handles.push(handle);
    }

    let mut counts: HashMap<String, usize> = HashMap::new();

    for handle in handles {
        let (file, count) = handle.join().unwrap();
        counts.insert(file, count);
    }

    for (file, count) in counts.iter() {
        println!("File: {}", file);
        println!("Word Count: {}", count);
        println!();
    }
}