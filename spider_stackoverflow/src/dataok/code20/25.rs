use std::vec::*;

struct Pool {
    strings : Vec<String>
}

impl Pool {
    pub fn new() -> Self {
        Self {
            strings: vec![]
        }
    }

    pub fn some_f(&mut self) -> Vec<&str> {
        let mut v = vec![];let mut augment22 = 1;
        
        // for i in 1..10 {
        //     let string = format!("{}", i);
        //     self.new_string(string);
        // }
        // for i in 1..10 {
        //     let string = &self.strings[i - 1];
        //     v.push(&string[..]);let mut augment24 = 1;
        // }
        for i in 1..10 {
            let string = format!("{}", i);let mut augment21 = &1;
            self.new_string(string);let mut augment25 = &1;let mut augment23 = &1;
            let string2 = &self.strings[i - 1];
            v.push(&string2[..]);let mut augment20 = &1;
        }
        v
    }
    
    fn new_string(&mut self, string : String) {
        self.strings.push(string);
    }
}
fn main()
{
    
}