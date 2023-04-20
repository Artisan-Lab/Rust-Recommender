fn main() {
    let mut s = String::from("hello");
    let s1 = &mut s;
    let s2 = s1;

    *s2 = String::from("world1");
    *s1 = String::from("world2");

    println!("{:?}", s);
}
// s1 转移给了s2 但是继续使用了 &mut 也遵循一个owner原理