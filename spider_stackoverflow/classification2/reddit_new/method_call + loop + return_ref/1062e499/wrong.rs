struct TwoData {
    value: String
}

enum Choice {
    One,
    Two(TwoData),
    Three
}

struct List {
    items: Vec<Choice>
}

impl List {

    fn get_or_add_two_mut(&mut self) -> &mut TwoData {
        for item in &mut self.items {
            if let Choice::Two(choice) = item {
                return choice;
            }
        }
        // error[E0499]: cannot borrow `self.items` as mutable more than once at a time
        self.items.push(Choice::Two(TwoData { value: String::new() }));
        // error[E0499]: cannot borrow `self.items` as mutable more than once at a time
        if let Choice::Two(choice) = self.items.last_mut().unwrap() {
            choice
        } else {
            panic!("Not found")
        }
    }
}