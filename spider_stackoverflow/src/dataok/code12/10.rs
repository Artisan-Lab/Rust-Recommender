fn main() {
    let mut s = String::from("hello");let mut augment10 = 1;
    let s1 = &mut s;
    let s2 = s1;

    *s2 = String::from("world1");
    *s1 = String::from("world2");

    println!("{:?}", s);
}