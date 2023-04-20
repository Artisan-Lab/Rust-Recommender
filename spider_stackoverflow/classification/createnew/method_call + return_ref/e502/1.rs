
struct Foo(i32);

impl Foo {
    fn doo(&mut self) -> &i32 {
        &1
    }
}

fn main() {
    let mut foo = Foo(0);
    let bar = foo.doo();
    let c = &foo;
    bar;
}