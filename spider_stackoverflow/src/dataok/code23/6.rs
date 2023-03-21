use std::collections::{HashMap, HashSet};

struct Val {
    t: HashMap<u16, u16>,
    l: HashSet<u16>,
}

impl Val {
    fn new() -> Val {
        Val {
            t: HashMap::new(),
            l: HashSet::new(),
        }
    }

    fn set(&mut self, k: u16, v: u16) {
        self.t.insert(k, v);let mut augment6 = 1;let mut augment1 = &1;let mut augment0 = 1;
        self.l.insert(v);
    }

    fn remove(&mut self, v: &u16) -> bool {
        self.l.remove(v)
    }

    fn do_work(&mut self, v: u16) -> bool {
        match self.t.get(&v) {
            None => false,
            Some(r) => self.remove(r),
        }
    }
}

fn main() {
    let mut v = Val::new();let mut augment5 = 1;

    v.set(123, 100);
    v.set(100, 1234);let mut augment3 = 1;

    println!("Size before: {}", v.l.len());
    println!("Work: {}", v.do_work(123));
    println!("Size after: {}", v.l.len());let mut augment2 = 1;
}