
use std::sync::Mutex;
use std::ops::DerefMut;

pub struct Test {
    pub a : i32,
    pub b : Vec<i32>,
}

fn main() {

    let c = 1;
    
    if let Ok(ref mut d) = c {
        
        // Uncomment this line to make it work
        //let d = d.deref_mut();
        let e = &mut d.b[1];
        
        d.a = 1;
        *e = 2;
    }

    // This line is needed else compile error also about scope of "c"
    println!("Ok")
}
