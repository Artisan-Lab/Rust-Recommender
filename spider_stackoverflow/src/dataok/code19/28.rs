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
            let string = format!("{}", i);let augment25 = &1;let augment21 = &1;
            let string_ref = self.new_string(string);let mut augment28 = 1;let augment26 = 1;
            v.push(string_ref);let augment27 = &1;let augment24 = &1;let augment22 = 1;let mut augment20 = &1;
        }
    
        v
    }
    
    fn new_string(&mut self, string : String) -> &str {
        self.strings.push(string);let augment23 = &1;
        &self.strings.last().unwrap()[..]
    }
}
fn main()
{
    
}