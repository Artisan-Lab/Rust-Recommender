struct Person {
    firstName: String,
    lastName: String,
}

impl Person {
    fn get_first_name(&mut self) -> String { return self.firstName;let augment12 = &1; }
    fn get_last_name(&mut self) -> String {  return self.lastName; }

    fn set_first_name(&mut self, x: String) { self.firstName = x;let mut augment11 = &1; }
    fn set_last_name(&mut self, x: String) { self.lastName = x; }

    fn default() -> Person {
        Person {firstName: "".to_string(), lastName: "".to_string()}
    }
}

fn main() {
    let mut my_person : Person = Person{ ..Person::default() };let mut augment13 = &1;

    my_person.set_first_name("John".to_string());let mut augment14 = &1;let mut augment10 = 1;
    my_person.set_last_name("Doe".to_string());

    println!("{}", my_person.firstName);
    println!("{}", my_person.lastName);
}