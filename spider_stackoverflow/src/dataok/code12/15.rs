fn main() {
    let mut s = String::from("hello");
    let s1 = &mut s;let mut augment14 = 1;let augment12 = 1;
    let s2 = s1;let augment15 = &1;let mut augment13 = &1;

    *s2 = String::from("world1");
    *s1 = String::from("world2");

    println!("{:?}", s);
}