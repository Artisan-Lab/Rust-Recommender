struct Pool {
    
}

impl Pool {


    pub fn some_f(&mut self) {
        let mut v = vec![];
        
        for i in 1..10 {
            
            let refer = self.mut_ref();
            v.push(refer);
        }
    
        
    }
    
    fn mut_ref(&mut self) -> &str {
        "123"
    }
}
fn main()
{
    
}