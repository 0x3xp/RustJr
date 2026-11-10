# Rust Filesystem Operations (file-ops)

A hands-on exploration of Rust's standard library filesystem API (`std::fs` and `std::path::Path`).

This project walks through practical disk I/O routines: directory inspection, recursive folder management, file creation, file deletion, and reading raw byte streams into memory using custom closures and iterator folding.

---

## What It Covers

1. **Path Existence Checking & Directory Creation (`test_create_dir`)**:
   - Inspects paths before operating using `std::path::Path::new(path).exists()`.
   - Creates directories dynamically using `std::fs::create_dir`.
   - Handles `Result` types and logs error diagnostics if directory allocation fails.

2. **File Writing & Selective Deletion (`create_files`)**:
   - Performs direct atomic writes using `std::fs::write`.
   - Cleans up temporary artifacts using `std::fs::remove_file`.

3. **Recursive Directory Removal (`remove_dir`)**:
   - Safely removes directory trees using `std::fs::remove_dir_all`.

4. **Raw Byte Reading & Functional Transformation (`read_file`)**:
   - Reads raw file contents into a byte vector (`Vec<u8>`) via `std::fs::read`.
   - Iterates and folds byte values into a `String` using a custom closure (`fold` accumulator), demonstrating low-level data transformation without high-level string convenience wrappers.

---

## Project Structure

```
file-ops/
└── file/
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs       # Entrypoint executing the filesystem pipeline
    │   └── myfs.rs       # Core filesystem operation module
    └── data/             # Working directory created at runtime for file I/O
```

---

## Building and Running

From the project root:

```bash
cd file
cargo run
```

### Expected Output

```text
Created new dir successfull: ./data.
Created new dir successfull: ./del
Successfully removed file: ./data/file2.txt
Successfully removed ./del
File read: ./data/file.txt includes content: "RUST RUST RUST RUST"
```

Running it a second time validates path existence checks:
```text
Directory already exists.
Successfully removed file: ./data/file2.txt
Problem occured can not remove dir: ./del
File read: ./data/file.txt includes content: "RUST RUST RUST RUST"
```

---

## Author

**Piyusha Akash**  
Portfolio: [https://0x3xp.github.io](https://0x3xp.github.io)

---

## Copyright & License

Copyright (c) 2026 Piyusha Akash. All rights reserved.  
Personal portfolio and projects: [0x3xp.github.io](https://0x3xp.github.io)
