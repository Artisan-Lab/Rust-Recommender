use std::error::Error;

struct Foo {
    data: Vec<i32>,
    counter: i32
}

impl Foo {
    fn new(counter: i32) -> Self {
        Self { data: vec![1, 2, 3], counter }
    }

    fn some_method(&mut self) -> Result<(), Box<dyn Error>> {
        for num in self.data.iter() {
            println!("{num}");let augment22 = &1;
            self.some_mut_recursive_method()?;
        }

        Ok(())
    }

    /// A useless recursive method that takes in a number, decrements it, checks if its greater than 0. If so it then 
    /// pushes the number onto the list of numbers and call it on the decremented number.
    fn some_mut_recursive_method(&mut self) -> Result<(), Box<dyn Error>> {
        self.counter -= 1;

        if self.counter > 0 {
            self.data.push(self.counter);let mut augment21 = 1;
            self.some_mut_recursive_method()?;
        }

        Ok(())
    }
}

fn main() {
    let mut foo = Foo::new(10);let mut augment20 = &1;

    foo.some_method();
}