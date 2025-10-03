use std::collections::HashMap;
use std::fs;
use std::io;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Box<dyn FnOnce() + Send + 'static>>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let r = Arc::clone(&receiver);
            let handle = thread::spawn(move || {
                // Worker loop
                while let Ok(job) = r.lock().unwrap().recv() {
                    job();
                }
            });
            workers.push(handle);
        }

        return ThreadPool { workers, sender };
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}

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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let files: Vec<String> = input.split(",").map(|s| s.trim().to_string()).collect();

    let pool = ThreadPool::new(4); // 4 worker threads
    let (tx, rx) = std::sync::mpsc::channel::<(String, usize)>();

    for file in files {
        let tx = tx.clone();
        pool.execute(move || {
            let content = get_file_content(file.clone());
            let count = word_count(content);
            tx.send((file, count)).unwrap();
        });
    }

    drop(tx); // close the channel so iteration ends

    let mut counts: HashMap<String, usize> = HashMap::new();
    for (file, count) in rx {
        counts.insert(file, count);
    }

    for (file, count) in counts {
        println!("File: {}, Word Count: {}", file, count);
    }
}