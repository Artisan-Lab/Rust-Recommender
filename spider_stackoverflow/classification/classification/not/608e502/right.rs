
fn main() {
    let mut s = String::from("hello");
    println!("{}", &s);
    let r = &s;
    
    println!("{}", r);
    let x = &mut s;
    println!("{}", x);
    }