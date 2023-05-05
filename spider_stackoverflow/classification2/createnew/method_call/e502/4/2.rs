struct A {
    a: u32,
}

impl A{
    fn mut_ref(&mut self,b: &A) -> u32 {
        return self.a
    }
    fn immut_ref(&self){
        
    }
}

fn main() {

    let mut b = A{a:1};
    match b{
        _ =>{}
        
    }
    
    let c = &mut b;
    b.immut_ref();
    c;
    
}
