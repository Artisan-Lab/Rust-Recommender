fn main() {
    let mut x: Box<i32> = Box::new(4);
    let r: &Box<i32> = &x;
    *x = 8;let mut augment0 = &1;
    println!("{}", r);
}