#![warn(dead_code)]
#![warn(unused_assignments)]
#[derive(Debug)]
pub struct Arguments {
    pub n:bool,
    pub b:bool,
    pub e:bool,
    pub file:String,

}

impl Arguments {
    pub fn build(arg:&Vec<String>) -> Arguments{
        let mut n = false;
        let mut b = false;
        let mut e = false;
        let mut file = String::new();
        for ar in arg{
            if ar == "-n" {
                n = true;
            }
            else if ar == "-e" {
                e= true;
            }
            else if ar == "-b" {
                b= true;
            }

        }
        file = arg[1].clone();

        Arguments {
            n,
            b,
            e,
            file
        }

    }
}