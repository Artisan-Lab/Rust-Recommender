fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = & s;

    println!("{}", r2);
}