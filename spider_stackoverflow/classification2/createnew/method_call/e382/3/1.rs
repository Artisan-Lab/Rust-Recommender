struct AB {
    a: u8,

}

impl AB {
    fn foo(&self) -> String {
        String::from("foo")
    }
    fn bar(self, x: String) -> String {
        String::from("foo")
    }
}

fn main() {
loop {
    let x = AB { a: 3 };
    let result = x.bar(x.foo());
    
}
}