#![warn(dead_code)]
#![warn(unused_assignments)]
#[derive(Debug)]
pub struct Arguments {
    pub w:bool,
    pub b:bool,
    pub l:bool,
    pub file:String,
}

impl Arguments {
    pub fn build(arg:&Vec<String>) -> Arguments {
        let mut w = false;
        let mut b = false;
        let mut l = false;
        let mut file = String::new();

        for a in arg {
            if a == "-w" {
                w = true;
            }
            else if a == "-b" {
                b = true;
            }
            else if a == "-l" {
                l = true;
            }
            else if !a.contains("-"){
                file = a.to_string();
            }
        }

        Arguments {
            w,
            b,
            l,
            file,
        }
    }
    
}