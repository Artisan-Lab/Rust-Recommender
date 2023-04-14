fn main() {
    let mut s = String::from("hello");
    let s1 = &mut s;
    let s2 = s1;

    *s2 = String::from("world1");let mut augment0 = 1;
    *s1 = String::from("world2");let augment1 = 1;

    println!("{:?}", s);
}