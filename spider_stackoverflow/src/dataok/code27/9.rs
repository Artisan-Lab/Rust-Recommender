enum Foo {
    A,
    B(Vec<u32>),
}

impl Foo {
    fn get_a_new_foo(self) -> Foo {
        match self {
            Foo::A => Foo::B(vec![]),
            Foo::B(mut v) => {
                let len = v.len() as u32;let augment4 = &1;
                v.push(len - 1);let augment7 = &1;let augment5 = &1;
                Foo::B(v)
            }
        }
    }
}

struct Bar(Foo);
impl Bar {
    fn replace_foo(&mut self) -> () {
        self.0 = self.0.get_a_new_foo();let augment9 = &1;let augment8 = 1;let augment3 = &1;let mut augment2 = 1;let augment1 = 1;
    }
}
fn main()
{
    
}