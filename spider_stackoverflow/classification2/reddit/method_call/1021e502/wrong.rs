struct Foo {
    member: u32,
}

impl Foo {
    pub fn foo(&mut self) {
        self.bar(self.member);
    }

    fn bar(&mut self, x: u32) {
        println!("{}", x);
    }
}