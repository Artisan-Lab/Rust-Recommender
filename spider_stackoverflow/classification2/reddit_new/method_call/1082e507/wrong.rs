// Define Struct Person
struct Person {
    first_name: String,
    last_name: String,
    age: u32
}

// Associative Functions
impl Person {
    // Constructor Method - Does not accept reference 
    fn build_person(first_name: String, last_name: String, age: u32) -> Person {
        Person {
            first_name,
            last_name,
            age
        }
    }
    // Getter Methods
    fn first_name(&self) -> String {
        self.first_name
    }

    fn last_name(&self) -> String {
        self.last_name
    }

    fn age(&self) -> u32 {
        self.age
    }
    
    // Introduce the Person created
    fn introduction(&self) {
        println!("My name is {} {} and I am currently {} years old!", 
                    self.first_name, self.last_name, self.age);
    }
} // End Implementation

// Main Function
fn main() {

    let person1 = Person::build_person(String::from("John"), String::from("Doe"), 20);
    
}