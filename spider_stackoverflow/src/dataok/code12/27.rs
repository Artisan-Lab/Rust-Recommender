fn main() {
    let mut s = String::from("hello");
    let s1 = &mut s;let augment25 = 1;
    let s2 = s1;let mut augment21 = &1;let mut augment20 = 1;

    *s2 = String::from("world1");let augment27 = &1;let mut augment24 = 1;let mut augment23 = 1;
    *s1 = String::from("world2");let mut augment22 = 1;

    println!("{:?}", s);let augment26 = 1;
}