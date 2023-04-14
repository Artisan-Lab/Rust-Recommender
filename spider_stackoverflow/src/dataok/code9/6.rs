fn main() {
    let mut x: Box<i32> = Box::new(4);
    let r: &Box<i32> = &x;let mut augment4 = 1;
    *x = 8;
    println!("{}", r);let augment6 = &1;let mut augment5 = &1;
}