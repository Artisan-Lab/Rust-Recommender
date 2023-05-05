use std::collections::HashMap;

struct Process {
    const_data: HashMap<String, i32>, // this is supposed to never be changed after construction!
    mut_data: i32, // this is supposed to be mutated during object lifetime
}


impl Process {
    fn do_more(&mut self, num: &i32) {
        self.mut_data += num;
    }

    fn do_somethig(&mut self, name: &str) {
        let num = self.const_data.get(name).unwrap(); // reference to part of self.const_data
        self.do_more(num); // borrowing &mut self
    }
}

fn main() {
    let mut p = Process { const_data: HashMap::from([
        ("foo".to_string(), 1),
        ("bar".to_string(), 2),
        ("buzz".to_string(), 3),
    ]), mut_data: 0 };

    p.do_somethig("foo");
}
// 可以理解为也是因为get导致的问题，getunwarp返回的东西是个option 里面的引用与自身生命周期相同