struct A {}

impl A {
    fn foo(&mut self, i: i32) -> i32 {
        i
    }
    
    fn bar(&mut self) -> i32 {
        1
    }
}

fn main() {
    let mut b = A{};
    b.foo(b.bar());
}