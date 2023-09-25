struct Foo {
    a: String,
}
impl Foo {
    fn bar(&mut self, text : &str) { // has to be mutable for some reason not shown here
        println!("{}", text);
    }
}
fn main() {
    let mut foo = Foo { a: String::from("aaa") };
    foo.bar("this is okay");
    foo.bar(& foo.a); // this is not
}