fn main() {
    let mut x: Box<i32> = Box::new(4);let augment1 = &1;let mut augment0 = &1;
    let r: &Box<i32> = &x;let augment4 = 1;let augment3 = 1;let mut augment2 = 1;
    *x = 8;let augment7 = &1;let mut augment5 = 1;
    println!("{}", r);let augment9 = 1;let mut augment8 = 1;let mut augment6 = 1;
}