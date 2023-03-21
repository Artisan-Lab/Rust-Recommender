fn main() {
    let mut x: Box<i32> = Box::new(4);let augment11 = &1;
    let r: &Box<i32> = &x;
    *x = 8;let mut augment13 = &1;let mut augment10 = &1;
    println!("{}", r);let augment12 = 1;
}