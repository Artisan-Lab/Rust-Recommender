struct Test {
}

impl Test {
    fn foo(&mut self, r: i32) -> i32 { r }
    fn bar(&mut self) -> i32 { 1 }
}

fn main() {
    let mut t = Test{};
    t.foo(t.bar());
}