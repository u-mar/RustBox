use std::env::args;
pub mod arg;
pub mod print;

fn main() {
    let arg:Vec<String> = args().collect();
    let a = arg::Arguments::build(&arg);
    if arg.contains(&"-h".to_string()) || arg.contains(&"--help".to_string()) {
        print_help();
        return;
    }
    print::print(a);
}

fn print_help() {
    println!("Usage: fastwc <file> [OPTIONS] ");
    println!();
    println!("Options:");
    println!("  -l        print the newline counts");
    println!("  -b        print the bytes");
    println!("  -w        print the word counts");
    println!("  -h, --help Show this help message");
}