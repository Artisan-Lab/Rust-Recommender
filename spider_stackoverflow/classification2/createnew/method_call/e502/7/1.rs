struct A {
    a: u32,
}

impl A{
    fn mut_ref(&mut self)  {
        loop {
        
        self.mut_ref2(&self.a);
        }
    }
    fn mut_ref2(&mut self,x:& u32){
        
    }
}

fn main() {

}