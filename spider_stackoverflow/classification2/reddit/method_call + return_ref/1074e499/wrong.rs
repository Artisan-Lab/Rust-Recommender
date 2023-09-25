struct Foobar {
}

impl Foobar {
    pub fn do_something(&mut self, _value: &mut i64) {
        println!("did something");
    }
}

struct Banana {
    data: Vec<Foobar>,
    index: usize,
    // Value to be passed to Foobar::do_something
    value: i64,
}

impl Banana {
    pub fn new() -> Banana {
        Banana {
            data: vec![Foobar{}],
            index: 0,
            value: 1234,
        }
    }

    pub fn get_ref_mut(&mut self) -> &mut Foobar {
        &mut self.data[self.index]
    }

    pub fn blah(&mut self) {
        // Works
        //self.data[self.index].do_something(&mut self.value);
        // Does not work
        self.get_ref_mut().do_something(&mut self.value);
    }
}

pub fn main() {
    Banana::new().blah();
}