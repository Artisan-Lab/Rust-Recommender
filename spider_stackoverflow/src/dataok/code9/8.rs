fn main() {
    let mut x: Box<i32> = Box::new(4);
    let r: &Box<i32> = &x;let mut augment8 = &1;
    *x = 8;
    println!("{}", r);
}