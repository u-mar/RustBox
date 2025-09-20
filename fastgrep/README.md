# Fastgrep

A simple, lightweight `grep`-like tool written in Rust.  Supports searching text patterns in files or standard input, with options for case-insensitivity, counting matches, and showing line numbers.

---

## Features
- `-h`, `--help` → Show usage information  
- `-i` → Case-insensitive search  
- `-c` → Show total count of matching lines  
- `-l` → Show line numbers along with matches  

---

## Installation

Clone this repository and build using Cargo:

```bash
git clone https://github.com/u-mar/Fastgrep.git
cd Fastgrep
cargo build --release

```
## Usage

```bash
fastgrep hello in books.txt
```
```bash
fastgrep -i -l hello in books.txt
```
```bash
fastgrep -c -i hello in books.txt
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.


## TODO

- [ ] Support **stdin** when no file is given (`cat file | fastgrep foo`)
- [ ] Handle **multiple files** and prefix matches with filename
- [ ] Add **invert match** option (`-v`) to show non-matching lines
- [ ] Add **whole word** option (`-w`)
- [ ] Add **whole line** option (`-x`)
- [ ] Add **regex support** (using `regex` crate)
- [ ] Add **recursive search** (`-r`) with `walkdir` crate
- [ ] Add option to show only **filenames with matches** (`-l`)
- [ ] Add option to show only **filenames without matches** (`-L`)
