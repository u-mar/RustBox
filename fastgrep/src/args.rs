#[allow(dead_code)]
use std::collections::HashMap;
#[derive(Debug)]
pub struct Arguments {
    pub i:bool,
    pub c:bool,
    pub l:bool,
    pub m:bool,
    pub word:String,
    pub file:String,

}
impl Arguments {
    pub fn build(arg:&Vec<String>) -> Arguments{
        let mut i = false;
        let mut c = false;
        let mut l = false;
        let mut m = false;
        let mut word = String::new();
        let mut file = String::new();
        let mut count = 0;
        let mut test = HashMap::new();
        for argu in arg {
            if argu == "-i" {
                i = true;
            }
            else if argu == "-c"{
                c = true;
            }
            else if argu == "-l"{
                l = true;
           }
           else if !argu.contains("-"){
            test.insert(count, argu);
            count += 1;
           }
           else if argu == "-m"{
            m = true;
            
       }

        }
        for (i,a) in test {
            if i == 1 {
                word = a.to_string();
            }
            else if i == 3 {
                file = a.to_string();
            }
        }

        Arguments{
            i,
            c,
            l,
            m,
            word,
            file
        }

    }
}
