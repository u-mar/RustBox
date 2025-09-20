#![warn(dead_code)]

pub mod search;
pub mod args;
use std::env::args;
fn main() {
    let arg:Vec<String> = args().collect();
    let a: args::Arguments = args::Arguments::build(&arg);
    if arg.contains(&"-h".to_string()) || arg.contains(&"--help".to_string()) {
        print_help();
        return;
    }
    println!("{:?}",a);
    search::search(&arg,a);


}

fn print_help() {
    println!("Usage: fastgrep [OPTIONS] <word> in <file>");
    println!();
    println!("Options:");
    println!("  -n        number all output lines");
    println!("  -c        Print count of matches");
    println!("  -l        Show line numbers");
    println!("  -m        Return max lines needed");
    println!("  -h, --help Show this help message");
}

