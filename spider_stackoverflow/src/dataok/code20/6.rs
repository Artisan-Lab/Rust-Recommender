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
        //     let string = format!("{}", i);let augment0 = 1;
        //     self.new_string(string);let mut augment2 = &1;
        // }
        // for i in 1..10 {
        //     let string = &self.strings[i - 1];
        //     v.push(&string[..]);let mut augment4 = &1;let augment3 = &1;
        // }
        for i in 1..10 {
            let string = format!("{}", i);let mut augment6 = &1;let augment5 = &1;
            self.new_string(string);
            let string2 = &self.strings[i - 1];
            v.push(&string2[..]);let augment1 = 1;
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