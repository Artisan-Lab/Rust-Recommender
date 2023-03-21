fn main() {
    let mut s = String::from("hello");
    println!("{}", &s);let augment1 = &1;let augment0 = 1;
    let r = &s;
    let x = &mut s;
    println!("{}", r);
    
    println!("{}", x);
}
