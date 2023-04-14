fn main() {
    let mut s = String::from("hello");
    println!("{}", &s);
    let r = &s;let mut augment0 = 1;
    let x = &mut s;let mut augment1 = 1;
    println!("{}", r);
    
    println!("{}", x);
}
