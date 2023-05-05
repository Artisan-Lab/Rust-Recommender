
struct Foo {
    data: Vec<i32>,
}

impl Foo {


    fn loop_method(&mut self)  {
        &self;
        for num in &self.data {
            self.mut_ref();
        }
    }
     fn mut_ref(&mut self) {

    }
}

fn main() {
 
}