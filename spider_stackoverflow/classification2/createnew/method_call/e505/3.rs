struct AB {
    a: u8,

}

impl AB {
    fn foo(&self,x:String) -> String {
        String::from("foo")
    }
    fn bar(self) -> String {
        String::from("foo")
    }
}

fn main() {
    let x = AB { a: 3 };
    let result = x.foo(x.bar());

}