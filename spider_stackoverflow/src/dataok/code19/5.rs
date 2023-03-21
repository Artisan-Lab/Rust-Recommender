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
        let mut v = vec![];let augment5 = &1;
        
        for i in 1..10 {
            let string = format!("{}", i);let augment2 = &1;let augment0 = &1;
            let string_ref = self.new_string(string);let mut augment1 = &1;
            v.push(string_ref);let augment4 = &1;let mut augment3 = &1;
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