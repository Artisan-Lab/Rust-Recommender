struct A {
    a: u32,
}

impl A{
    fn mut_ref(&mut self,b: &A) -> u32 {
        return self.a
    }
    fn immut_ref(&self,b: &mut A){
        
    }
}

fn main() {
loop{
    let mut b = A{a:1};
    let c = &mut b;
    b.immut_ref(c);
    b;
    }
}
