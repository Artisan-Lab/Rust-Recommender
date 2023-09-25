struct Test<'a> {
    info: &'a str,
}

impl<'a> Test<'a> {
    fn build(&'a mut self) {}
    fn write(&self) {}
}

fn main() {
    let mut test = Test { info: "info", };
    test.build();
    test.write();
}