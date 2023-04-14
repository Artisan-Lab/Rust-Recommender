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
            println!("{num}");
            self.some_mut_recursive_method()?;
        }

        Ok(())
    }

    /// A useless recursive method that takes in a number, decrements it, checks if its greater than 0. If so it then 
    /// pushes the number onto the list of numbers and call it on the decremented number.
    fn some_mut_recursive_method(&mut self) -> Result<(), Box<dyn Error>> {
        self.counter -= 1;

        if self.counter > 0 {
            self.data.push(self.counter);
            self.some_mut_recursive_method()?;
        }

        Ok(())
    }
}

fn main() {
    let mut foo = Foo::new(10);

    foo.some_method();
}
// 没办法 只能用ref包起来，内部可变 然后再传，因为iter()必须在循环里持续有引用