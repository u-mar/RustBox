# Fastcat

A simple, lightweight `cat`-like tool written in Rust.  
Supports displaying file contents with options to number lines, number non-empty lines, and show end-of-line markers.

---

## Features
- `-h`, `--help` → Show usage information  
- `-n` → Number all output lines  
- `-b` → Number nonempty output lines (overrides `-n`)  
- `-e` → Display `$` at the end of each line  

---

## Installation

Clone this repository and build using Cargo:

```bash
git clone https://github.com/u-mar/Fastcat.git
cd Fastcat
cargo build --release
```
## Usage

```bash
fastcat Goals
```
```bash
fastcat Goals -b
```
```bash
fastcat Goals -n -e
```