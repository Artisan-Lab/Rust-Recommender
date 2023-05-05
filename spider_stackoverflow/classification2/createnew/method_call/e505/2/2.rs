

struct A (i32);
struct B{a:A}


impl A{
    fn act(self){
        
    }
}





fn main() {
    let mut p = A(1);
    let ref_p = &p;
    
    match ref_p {
        x =>{}
    }
    
    let mut b = B{a:p};
    b.a.act();
    ref_p;
    
}