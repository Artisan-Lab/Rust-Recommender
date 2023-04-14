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
        
        for i in 1..10 {
            let string = format!("{}", i);
            let string_ref = self.new_string(string);let mut augment15 = &1;let mut augment14 = &1;let augment12 = &1;
            v.push(string_ref);let mut augment13 = &1;
        }
    
        v
    }
    
    fn new_string(&mut self, string : String) -> &str {
        self.strings.push(string);
        &self.strings.last().unwrap()[..]
    }
}
fn main()
{
    
}