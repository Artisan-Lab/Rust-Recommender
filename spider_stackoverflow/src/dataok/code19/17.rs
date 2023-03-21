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
        let mut v = vec![];let mut augment13 = 1;
        
        for i in 1..10 {
            let string = format!("{}", i);let mut augment15 = &1;let augment14 = &1;let mut augment11 = &1;
            let string_ref = self.new_string(string);
            v.push(string_ref);let mut augment17 = &1;
        }
    
        v
    }
    
    fn new_string(&mut self, string : String) -> &str {
        self.strings.push(string);let mut augment16 = &1;let mut augment12 = &1;let mut augment10 = &1;
        &self.strings.last().unwrap()[..]
    }
}
fn main()
{
    
}