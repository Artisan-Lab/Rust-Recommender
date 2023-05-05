struct A {
    a: u32,
}

impl A{
    fn mut_ref(&mut self)  {
        self.mut_ref2(&mut self.a);
    }
    fn mut_ref2(&self,x:&mut u32){
        
    }
}

fn main() {

}
