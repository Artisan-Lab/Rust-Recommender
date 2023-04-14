fn main() {
    let mut s = String::from("hello");
    println!("{}", &s);
    let r = &s;let mut augment0 = 1;
    let x = &mut s;
    println!("{}", r);
    
    println!("{}", x);
}
