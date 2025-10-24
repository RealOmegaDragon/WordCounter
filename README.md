# WordCounter
*A simple Rust-based multithreaded tool demonstrating a custom thread pool and parallel file word counting.*

## Overview
**WordCounter** is a lightweight command-line program written in Rust that counts the total number of words in one or more text files **concurrently**. It showcases a **custom-built thread pool** implementation to manage and distribute work efficiently across multiple threads.

The project highlights Rust’s powerful **concurrency model** by leveraging ownership, channels, and synchronization primitives to safely perform parallel computation without external dependencies.

## Purpose
This project was created to demonstrate how thread pools can be implemented from scratch in Rust while maintaining **thread safety** and **data consistency**. It offers a hands-on example of how multi-threaded architectures can speed up I/O-bound tasks such as reading and processing multiple files.

Beyond its educational value, WordCounter can also serve as a base for more advanced concurrent file processing utilities.

## Features
- Reads and counts words across multiple text files in parallel.
- Implements a **custom thread pool** for controlled concurrency.
- Uses **channels (mpsc)** for safe inter-thread communication.
- Automatically collects and displays results for all processed files.
- Demonstrates **Arc**, **Mutex**, and **ownership management** in multi-threaded Rust.

## Built With
- **Rust** (Edition 2024)
- Standard library (`std::thread`, `std::sync`, `std::fs`, `std::io`) — *no external crates required*

## Project Structure
```
WordCounter/
│
├── src/
│   └── main.rs        # Main logic and custom ThreadPool implementation
│
├── Cargo.toml         # Project manifest
└── Cargo.lock         # Dependency resolution (auto-generated)
```

## Requirements
- **Rust 1.80**+
- One or more valid text files to count words from.

## Running the Project
1. Clone the repository:
   ```bash
   git clone https://github.com/RealOmegaDragon/WordCounter.git
   cd WordCounter
   ```
2. Build the binary:
   ```bash
   cargo build --release
   ```
3. Run the program:
   ```bash
   ./target/release/WordCounter
   ```
4. Enter one or more file paths separated by commas when prompted:
   ```
   example1.txt, example2.txt, notes.txt
   ```
The program will spawn worker threads, count words in each file concurrently, and print the results.

## Example
### Input:
```
example1.txt, example2.txt
```
### Output:
```
File: example1.txt, Word Count: 1532
File: example2.txt, Word Count: 987
```

## What I Learned
Building WordCounter provided deeper understanding of:
- Designing a **thread pool** from first principles.
- Using **message passing (mpsc)** for task delegation.
- Applying **Arc** and **Mutex** for safe shared-state management.
- Leveraging **Rust’s ownership and concurrency** guarantees for reliable parallel processing.

## Author
**Braxton Newhall**  
Social: [LinkedIn](https://linkedin.com/in/braxton-newhall-128597333) • [GitHub](https://github.com/RealOmegaDragon)  
Email: braxtonnewhall@gmail.com

## License
This project is open-source under the [MIT License](LICENSE).
