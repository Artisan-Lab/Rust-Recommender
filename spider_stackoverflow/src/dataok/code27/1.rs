enum Foo {
    A,
    B(Vec<u32>),
}

impl Foo {
    fn get_a_new_foo(self) -> Foo {
        match self {
            Foo::A => Foo::B(vec![]),
            Foo::B(mut v) => {
                let len = v.len() as u32;
                v.push(len - 1);
                Foo::B(v)
            }
        }
    }
}

struct Bar(Foo);
impl Bar {
    fn replace_foo(&mut self) -> () {
        self.0 = self.0.get_a_new_foo();let augment1 = 1;
    }
}
fn main()
{
    
}