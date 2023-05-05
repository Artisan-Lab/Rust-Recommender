
struct Pool {
    a : i32
}

impl Pool {


    pub fn some_f(&mut self) {
        
        loop {
            self.take_onwer();
        }
    }
    
    fn take_onwer( self) {
        
    }
}
fn main()
{
    
}