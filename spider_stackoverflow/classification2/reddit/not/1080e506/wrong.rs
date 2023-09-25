#[derive(Debug)]

struct Thing<'a> {

high: &'a f64,

low: &'a f64,

}

impl Thing<'_> {

pub fn product(&self) -> f64 {

*self.high * *self.low

}

}

fn main() {

    let mut x: f64 = 42.01;
    
    let y: f64 = 7.95;
    
    let w: f64 = 99.9;
    
    let mut thing1: Thing = Thing { high: &w, low: &x };
    
    let mut thing2: Thing = Thing { high: &y, low: &x };
    
    println!("Before {:?} {}", thing1, thing1.product());
    
    println!("Before {:?} {}", thing2, thing2.product());
    
    // I want to swap or copy in a new value for the reference x, because
    
    // it appears multiple times in different `Thing`s.
    
    x = 500.7_f64;
    
    println!("After {:?}", thing1);
    
    println!("After {:?}", thing2);

}