fn main() {
    let mut s = String::from("hello");
    println!("{}", &s);
    let r = &s;let mut augment4 = 1;
    let x = &mut s;
    println!("{}", r);let mut augment5 = 1;
    
    println!("{}", x);
}
