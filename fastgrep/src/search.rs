use std::fs;
use colored::Colorize;

use crate::args::Arguments;


pub fn search(arg: &Vec<String>, a: Arguments) {
    if arg.len() < 3 {
        panic!("Not enough Arguments");
    }

    let word = a.word.clone();
    let file = a.file.clone();
    println!("Searching {} in {}", word, file);

    let texts = fs::read_to_string(&file).unwrap();
    let mut total = 0;
    let mut found_any = false;

    for (i, line) in texts.lines().enumerate() {
        if let Some(highlighted) = doe(line, &a, &word) {
            found_any = true;
            total += 1;
            
            if !a.c {
                if a.l {
                    println!("{}: {}", (i + 1).to_string().blue(), highlighted);
                } else {
                    println!("{}", highlighted);
                }
            }
        }
    }

    if a.c {
        println!("{} matches found", total);
    }

    if !found_any {
        println!("{}", "No matches found".red());
    }
}


pub fn doe(line: &str, a: &Arguments, word: &str) -> Option<String> {
    let (line_check, word_check) = if a.i {
        (line.to_lowercase(), word.to_lowercase())
    } else {
        (line.to_string(), word.to_string())
    };

    if line_check.contains(&word_check) {
        let highlighted = line_check.replace(&word_check, &word_check.green().bold().to_string());
        Some(highlighted)
    } else {
        None
    }

}
