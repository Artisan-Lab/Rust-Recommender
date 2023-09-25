struct Foo<'a> {
    i: &'a i8,
}

impl<'a> Foo<'a> {
    fn bar(&'a mut self) {
        
    }
}

fn main() {
  let i = 0;
  let mut foo = Foo { i:&i };
  foo.bar();
  let _borrow = &foo;
}