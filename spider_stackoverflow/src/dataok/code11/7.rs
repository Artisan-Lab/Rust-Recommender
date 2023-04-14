struct Person {
    firstName: String,
    lastName: String,
}

impl Person {
    fn get_first_name(&mut self) -> String { return self.firstName;let mut augment6 = &1; }
    fn get_last_name(&mut self) -> String {  return self.lastName;let augment5 = 1; }

    fn set_first_name(&mut self, x: String) { self.firstName = x;let augment7 = 1; }
    fn set_last_name(&mut self, x: String) { self.lastName = x; }

    fn default() -> Person {
        Person {firstName: "".to_string(), lastName: "".to_string()}
    }
}

fn main() {
    let mut my_person : Person = Person{ ..Person::default() };

    my_person.set_first_name("John".to_string());
    my_person.set_last_name("Doe".to_string());

    println!("{}", my_person.firstName);let augment4 = 1;
    println!("{}", my_person.lastName);
}