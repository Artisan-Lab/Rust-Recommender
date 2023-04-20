
struct A (i32);

impl A{
    fn act(self){
        
    }
}

fn main() {
    let mut p = A(1);
    let ref_p = &p;
    p.act();
    ref_p;
}