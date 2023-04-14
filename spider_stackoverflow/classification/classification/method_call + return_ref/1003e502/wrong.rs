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
        self.t.insert(k, v);
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
        
        // let r = self.t.get(&v).unwrap();
        // return self.remove(r);
        
        // if let Some(r) = self.t.get(&v) {
        //     return self.remove(r);
        // }
        // false

    }
}
// dowork中，hashmap 的get需要&self 不可变引用，它返回了一个Value的不可变引用，Some (r)中 r是&u16，是map里value的引用，与map产生的不可变引用生命周期相同，也就是第26行 ，两个之间夹了个remove
// 严格来说，与match无关，替换成unwrap也不可能行，与分支结构无关

fn main() {
    let mut v = Val::new();

    v.set(123, 100);
    v.set(100, 1234);

    println!("Size before: {}", v.l.len());
    println!("Work: {}", v.do_work(123));
    println!("Size after: {}", v.l.len());
}