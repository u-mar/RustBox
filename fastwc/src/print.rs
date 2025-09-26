use std::fs;
use crate::arg::Arguments;


pub fn print(a:Arguments) {
    let file = a.file;
    let texts = fs::read_to_string(&file).unwrap();
    let line = texts.lines().count();
    let word= texts.split_ascii_whitespace().count();
    let byte = texts.len() + 1;
    
    if !a.l && !a.b && !a.w {
        println!("lines: {} words: {} bytes: {}",line,word,byte);
    }
    else {
        [
        (a.l, "lines", line),
        (a.w, "words", word),
        (a.b, "bytes", byte),
    ].into_iter().filter(|(enabled,_,_)| *enabled ).for_each(|(_,name,value)| print!("{}: {} ",name,value));
    println!();
    }



}