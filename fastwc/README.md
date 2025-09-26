# Fastwc

A simple, lightweight `wc`-like tool written in Rust.  
Supports counting lines, words, and bytes in files, similar to GNU `wc`.

---

## Features
- `-h`, `--help` → Show usage information  
- `-l` → Count the number of lines  
- `-w` → Count the number of words  
- `-b` → Count the number of bytes  

---

## Installation

Clone this repository and build using Cargo:

```bash
git clone https://github.com/u-mar/RustBox.git
cd fastwc
cargo build --release

```

## Usage

```bash
fastwc Goals
```
```bash
fastwc -l -w Goals
```
```bash
fastwc -b Goals
```

## TODO

- [ ] Support **multiple files** (fastcat file1 file2)
- [ ]  Add option to read from stdin when no file is given (fastcat file | fastwc)
- [ ]  Add error handling for unreadable or missing files