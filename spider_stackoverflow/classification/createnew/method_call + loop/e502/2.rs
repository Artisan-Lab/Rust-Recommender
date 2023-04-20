
struct Foo {
    data: Vec<i32>,
}

impl Foo {


    fn loop_method(&mut self)  {
        for num in &mut self.data {
            self.immut_ref();
        }
    }

     fn immut_ref(&self) {

    }
}

fn main() {
 
}