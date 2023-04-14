use std::{collections::HashMap, rc::Rc};

struct Process {
    const_data: Rc<HashMap<String, i32>>, // this is supposed to never be changed after construction!
    mut_data: i32,                        // this is supposed to be mutated during object lifetime
}

impl Process {
    fn do_more(&mut self, num: &i32) {
        self.mut_data += num;
    }

    fn do_somethig(&mut self, name: &str) {
        let map = Rc::clone(&self.const_data);
        let num = map.get(name).unwrap(); // reference to part of self.const_data
        self.do_more(num); // borrowing &mut self
    }
}

fn main() {
    let mut p = Process {
        const_data: Rc::new(HashMap::from([
            ("foo".to_string(), 1),
            ("bar".to_string(), 2),
            ("buzz".to_string(), 3),
        ])),
        mut_data: 0,
    };

    p.do_somethig("foo");
}
