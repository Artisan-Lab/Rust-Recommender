fn main() {
    let mut s = String::from("hello");
    let s1 = &mut s;
    let s2 = s1;

    *s2 = String::from("world1");let augment6 = &1;let augment5 = 1;
    *s1 = String::from("world2");let mut augment4 = 1;

    println!("{:?}", s);
}