struct Foo {
    inner: i32,
}

impl Foo {
    fn get_inner_mut(&mut self) -> &mut i32 { &mut self.inner }
    fn do_thing(&self, num: &i32) { /* whatever */ }
}

fn main() {
  let mut foo = Foo { inner: 12 };

  let bar: &i32 = {
    let baz = foo.get_inner_mut();
    &*baz
  };

  foo.do_thing(bar);
}