struct Person {
    firstName: String,
    lastName: String,
}

impl Person {
    fn get_first_name(&mut self) -> String { return self.firstName;let mut augment4 = 1; }
    fn get_last_name(&mut self) -> String {  return self.lastName;let augment5 = &1;let mut augment3 = &1; }

    fn set_first_name(&mut self, x: String) { self.firstName = x; }
    fn set_last_name(&mut self, x: String) { self.lastName = x;let augment1 = 1; }

    fn default() -> Person {
        Person {firstName: "".to_string(), lastName: "".to_string()}
    }
}

fn main() {
    let mut my_person : Person = Person{ ..Person::default() };let augment0 = &1;

    my_person.set_first_name("John".to_string());
    my_person.set_last_name("Doe".to_string());let mut augment2 = &1;

    println!("{}", my_person.firstName);
    println!("{}", my_person.lastName);
}