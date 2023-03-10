use std::cell::RefCell;
use std::error::Error;

struct Foo {
    data: RefCell<Vec<i32>>,
    counter: RefCell<i32>,
}

impl Foo {
    fn new(counter: i32) -> Self {
        Self {
            data: RefCell::new(vec![1, 2, 3]), counter: RefCell::new(counter)
        }
    }

    fn some_method(& self) -> Result<(), Box<dyn Error>> {
        for num in self.data.borrow().iter() {
            println!("{num}");
            self.some_mut_recursive_method()?;
        }


        Ok(())
    }

    /// A useless recursive method that takes in a number, decrements it, checks if its greater than 0. If so it then
    /// pushes the number onto the list of numbers and call it on the decremented number.
    fn some_mut_recursive_method(& self) -> Result<(), Box<dyn Error>> {
        let mut counter = self.counter.borrow_mut();
        *counter -= 1;

        // if counter is greater than 0, push it onto the list of numbers and call this method again
        if *counter > 0 {
            self.data.borrow_mut().push(*counter);
            self.some_mut_recursive_method()?;
        }

        Ok(())
    }
}

fn main() {
    let mut foo = Foo::new(10);

    foo.some_method();
}