use std::marker::PhantomData;

struct Child<'a> {
    _parent: PhantomData<&'a Parent>
}
struct Parent;

impl<'a> Child<'a> {
    pub fn new(_parent: &Parent) -> Child { Child { _parent: PhantomData} }
    pub fn foo(&mut self) {}
}

impl Parent {
    pub fn new() -> Parent { Parent{} }
    pub fn create(&self) -> Child { Child::new(self) }
    pub fn foo(&mut self) {}
}

fn main() {
    let mut parent = Parent::new();
    let mut child = parent.create();

    parent.foo();
    child.foo();
}