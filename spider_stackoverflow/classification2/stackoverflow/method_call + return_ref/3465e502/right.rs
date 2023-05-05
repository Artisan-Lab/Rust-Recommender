struct A {
    value: i64
}

impl A {
    pub fn new() -> Self {
        A { value: 0 }
    }
    pub fn do_something(&mut self, other: &B) {
        self.value += other.value;
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

struct B {
    pub value: i64
}
    
struct State {
    a: A,
    b: B
}

impl State {
    pub fn new() -> Self {
        State {
            a: A::new(),
            b: B { value: 1 }
        }
    }
    pub fn do_stuff(&mut self) -> i64 {
        self.a.do_something(&self.b);
        self.a.value()
    }
    // pub fn do_stuff(&mut self) -> i64 {
    //     self.a.do_something(self.get_b());
    //     self.a.value()
    // }
    
    pub fn get_b(&self) -> &B {
        &self.b
    }
}

fn main() {
    let mut state = State::new();
    println!("{}", state.do_stuff());
}