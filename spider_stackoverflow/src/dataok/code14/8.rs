fn main() {
    let mut s = String::from("hello");
    println!("{}", &s);
    let r = &s;
    let x = &mut s;let augment8 = 1;
    println!("{}", r);
    
    println!("{}", x);
}
