#[derive(Default)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Immutable access.
    fn first_name(&self) -> &str {
        &self.first_name
    }
    fn last_name(&self) -> &str {
        &self.last_name
    }

    // Mutable access.
    fn first_name_mut(&mut self) -> &mut String {
        &mut self.first_name
    }
    fn last_name_mut(&mut self) -> &mut String {
        &mut self.last_name
    }
}

fn main() {
    let mut my_person = Person::default();

    *my_person.first_name_mut() = String::from("John");
    *my_person.last_name_mut() = "Doe".into();

    println!("first_name: {}", my_person.first_name());
    println!("last_name: {}", my_person.last_name());
    
    // Can't do this efficiently with getter/setter!
    {
        let s = my_person.last_name_mut();
        s.truncate(2);
        s.push('w');
    }

    println!("first_name: {}", my_person.first_name());
    println!("last_name: {}", my_person.last_name());
}