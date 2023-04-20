struct ABC {
    a: Vec<i32>,    // no Copy trait
    b: i32,
}

impl ABC {
    fn new() -> ABC { ABC {a: vec![0,1], b: 10} }

    fn first(&mut self) {
        self.a.push(2);         // updates one field
        self.second(self.a.to_vec());
    }

    fn second(&mut self, x: Vec<i32>) {
        self.b += x[1];
    }
}

fn main() {
    let mut s = ABC::new();
    s.first();
}