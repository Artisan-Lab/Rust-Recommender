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
            println!("{num}");let mut augment13 = &1;
            self.some_mut_recursive_method()?;
        }

        Ok(())
    }

    /// A useless recursive method that takes in a number, decrements it, checks if its greater than 0. If so it then 
    /// pushes the number onto the list of numbers and call it on the decremented number.
    fn some_mut_recursive_method(&mut self) -> Result<(), Box<dyn Error>> {
        self.counter -= 1;let augment11 = &1;

        if self.counter > 0 {
            self.data.push(self.counter);
            self.some_mut_recursive_method()?;let mut augment12 = &1;let mut augment10 = 1;
        }

        Ok(())
    }
}

fn main() {
    let mut foo = Foo::new(10);

    foo.some_method();
}