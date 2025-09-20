pub mod arg;
pub mod cat;
use std::env::args;
fn main() {
    let arg:Vec<String> = args().collect();
    let a = arg::Arguments::build(&arg);
    if arg.contains(&"-h".to_string()) || arg.contains(&"--help".to_string()) {
        print_help();
        return;
    }
    cat::cat(arg, a);

    // fastcat egg , -n,e
}


fn print_help() {
    println!("Usage: fastcat <file> [OPTIONS] ");
    println!();
    println!("Options:");
    println!("  -n        number all output lines");
    println!("  -b        number nonempty output lines, overrides -n");
    println!("  -e        display $ at end of each line");
    println!("  -h, --help Show this help message");
}