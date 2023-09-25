struct A<'a> {
    some_str: &'a str
}

impl<'a> A<'a> {
    pub fn do_something(&'a self) {
        println!("{}", self.some_str);
    }

    pub fn do_something_mut(&'a mut self){
        println!("{}", self.some_str);
        self.some_str = "OOK";
    }
}

fn main() {
    let mut a = A{some_str: "adf"};

    a.do_something_mut();
    a.do_something();
}