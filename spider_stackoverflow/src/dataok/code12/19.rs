fn main() {
    let mut s = String::from("hello");
    let s1 = &mut s;let mut augment17 = 1;
    let s2 = s1;let mut augment18 = 1;

    *s2 = String::from("world1");let augment16 = &1;
    *s1 = String::from("world2");let augment19 = &1;

    println!("{:?}", s);
}