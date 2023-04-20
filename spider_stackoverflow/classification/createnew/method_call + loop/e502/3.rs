use std::vec::*;

struct Pool {
    a : i32
}

impl Pool {


    pub fn some_f(&mut self)  {
        let mut v = vec![];
        
        // for i in 1..10 {
        //     self.mut_ref();
        // }
        // for i in 1..10 {
        //     let immut_ref = &self.a;
        //     v.push(immut_ref);
        // }
        
        for i in 1..10 {
            
            self.mut_ref();
            
            let immut_ref = &self.a;
            v.push(immut_ref);
        }
        
    }
    
    fn mut_ref(&mut self) {
        
    }
}
fn main()
{
    
}