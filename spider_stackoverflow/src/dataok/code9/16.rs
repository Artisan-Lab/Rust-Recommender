fn main() {
    let mut x: Box<i32> = Box::new(4);
    let r: &Box<i32> = &x;
    *x = 8;let mut augment16 = &1;
    println!("{}", r);
}