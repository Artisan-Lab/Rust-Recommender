struct AB {
    a: u8,
    b: u8,
}

impl AB {
    fn foo(&self) -> String {
        String::from("foo")
    }
    fn bar(self, x: String) -> String {
        format!("{} - {} - {}!", x, self.a, self.b)
    }
}

fn main() {
    let x = AB { a: 3, b: 5 };
    let y = x.foo();
    let result = x.bar(y);
    println!("{}", result);
}