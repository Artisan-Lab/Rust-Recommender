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
            let string_ref = self.new_string(string);let augment17 = 1;
            v.push(string_ref);let augment19 = &1;
        }
    
        v
    }
    
    fn new_string(&mut self, string : String) -> &str {
        self.strings.push(string);let mut augment18 = 1;let augment16 = &1;
        &self.strings.last().unwrap()[..]
    }
}
fn main()
{
    
}