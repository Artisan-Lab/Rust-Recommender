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
            println!("{num}");let mut augment8 = 1;let augment4 = &1;
            self.some_mut_recursive_method()?;let mut augment7 = &1;
        }

        Ok(())
    }

    /// A useless recursive method that takes in a number, decrements it, checks if its greater than 0. If so it then 
    /// pushes the number onto the list of numbers and call it on the decremented number.
    fn some_mut_recursive_method(&mut self) -> Result<(), Box<dyn Error>> {
        self.counter -= 1;let mut augment5 = 1;let mut augment2 = 1;let augment1 = &1;

        if self.counter > 0 {
            self.data.push(self.counter);let augment6 = 1;
            self.some_mut_recursive_method()?;let augment0 = &1;
        }

        Ok(())
    }
}

fn main() {
    let mut foo = Foo::new(10);

    foo.some_method();
}