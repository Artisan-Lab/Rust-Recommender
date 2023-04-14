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
            let string = format!("{}", i);let augment9 = &1;
            let string_ref = self.new_string(string);
            v.push(string_ref);let mut augment8 = &1;
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