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
        let mut v = vec![];
        
        // for i in 1..10 {
        //     let string = format!("{}", i);
        //     self.new_string(string);let augment4 = &1;
        // }
        // for i in 1..10 {
        //     let string = &self.strings[i - 1];
        //     v.push(&string[..]);
        // }
        for i in 1..10 {
            let string = format!("{}", i);
            self.new_string(string);let augment5 = 1;
            let string2 = &self.strings[i - 1];let augment6 = 1;
            v.push(&string2[..]);
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