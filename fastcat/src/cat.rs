use std::fs;
use crate::arg::Arguments;

pub fn cat(arg:Vec<String>,a:Arguments){
    if arg.len() < 2 {
        eprintln!("Usage: fastcat <file>");
        std::process::exit(1);
    }
    let file = a.file;

    let texts = fs::read_to_string(&file).unwrap();
    for (i,text) in texts.lines().filter(|text| if a.b {!text.trim().is_empty()} else {true}).enumerate(){
        if a.e {
            println!("{} {:}$",i+1,text);
        }
        else {
            println!("{} {:}",i+1,text);
        }

    }
}