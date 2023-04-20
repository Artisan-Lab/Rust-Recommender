struct A {
    a: u32,
}

impl A{
    fn mut_ref(&mut self,a: &mut A)  {
        // return self.a
    }
    fn mut_ref2(&mut self){
        
    }
}

fn main() {
    let mut b = A{a:1};
    let c = &mut b;
    b.mut_ref(c);

    
}
