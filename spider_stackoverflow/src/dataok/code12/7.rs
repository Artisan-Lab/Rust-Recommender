fn main() {
    let mut s = String::from("hello");let mut augment2 = 1;let augment1 = &1;
    let s1 = &mut s;
    let s2 = s1;let mut augment6 = 1;let mut augment4 = &1;let augment3 = 1;let mut augment0 = 1;

    *s2 = String::from("world1");
    *s1 = String::from("world2");let mut augment7 = &1;

    println!("{:?}", s);let augment5 = &1;
}