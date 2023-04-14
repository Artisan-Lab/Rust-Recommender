fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = & s;let augment12 = 1;

    println!("{}", r1);
}