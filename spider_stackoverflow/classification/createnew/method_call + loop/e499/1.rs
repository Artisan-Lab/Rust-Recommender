
struct Foo {
    data: Vec<i32>,
}

impl Foo {


    fn loop_method(&mut self)  {
        for num in &mut self.data {
            self.mut_ref();
        }
    }

     fn mut_ref(&mut self) {

    }
}

fn main() {
 
}