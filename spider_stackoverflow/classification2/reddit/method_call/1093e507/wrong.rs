#[derive(Debug)]
struct Animal {
    food: String,
    age: i32,
}

#[derive(Debug)]
struct AnimalBuilder {
    food: String,
    age: i32,
}

impl AnimalBuilder {
    fn new() -> AnimalBuilder {
        AnimalBuilder {
            food: "".to_string(),
            age: 0,
        }
    }

    fn food(&mut self, f: &str) -> &mut AnimalBuilder {
        self.food = f.to_string();
        self
    }

    fn age(&mut self, a: i32) -> &mut AnimalBuilder {
        self.age = a;
        self
    }

    fn finalize(&self) -> Animal {
        Animal {
            food: self.food, // error: cannot move out of borrowed content [E0507]
            age: self.age,
        }
    }
}

fn main() {
    let panda = AnimalBuilder::new()
        .food("Bamboo")
        .age(10)
        .finalize();

    println!("{:?}", panda);
}
