fn main() {
    let mut x: Box<i32> = Box::new(4);let augment1 = &1;let mut augment0 = &1;
    let r: &Box<i32> = &x;let mut augment2 = 1;
    *x = 8;
    println!("{}", r);
}