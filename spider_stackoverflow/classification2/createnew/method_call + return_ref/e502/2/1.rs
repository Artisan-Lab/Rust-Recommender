
struct Foo(i32);

impl Foo {
    fn doo(&mut self) -> &i32 {
        &1
    }
    fn doo2(&self,a:&i32) {
        
    }
}

fn main() {
loop {
    let mut foo = Foo(0);
    let bar = foo.doo();
    foo.doo2(bar);
    }
}