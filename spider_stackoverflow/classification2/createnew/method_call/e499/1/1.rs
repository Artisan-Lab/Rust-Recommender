struct A {
    a: u32,
}

impl A{
    fn mut_ref(&mut self)  {
        // return self.a
    }
    fn immut_ref(&self){
        
    }
}

fn main() {
    loop {
    let mut b = A{a:1};
    let c = &mut b;
    b.mut_ref();
    c;
    }
    
}
