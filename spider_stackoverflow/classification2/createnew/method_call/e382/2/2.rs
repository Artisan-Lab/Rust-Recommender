

struct A (i32);
struct B{a:A}


impl A{
    fn act(self){
        
    }
}





fn main() {
loop {
    let mut p = A(1);
    let mut b = B{a:p};
    b.a.act();
    b;
    b;
    }
}