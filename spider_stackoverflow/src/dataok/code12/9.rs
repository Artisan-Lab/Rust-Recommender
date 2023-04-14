fn main() {
    let mut s = String::from("hello");
    let s1 = &mut s;let mut augment8 = &1;
    let s2 = s1;

    *s2 = String::from("world1");let augment9 = &1;
    *s1 = String::from("world2");

    println!("{:?}", s);
}