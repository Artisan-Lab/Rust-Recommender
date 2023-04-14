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
            let string = format!("{}", i);let mut augment6 = 1;
            let string_ref = self.new_string(string);let augment4 = 1;
            v.push(string_ref);
        }
    
        v
    }
    
    fn new_string(&mut self, string : String) -> &str {
        self.strings.push(string);let augment7 = &1;let mut augment5 = 1;
        &self.strings.last().unwrap()[..]
    }
}
fn main()
{
    
}