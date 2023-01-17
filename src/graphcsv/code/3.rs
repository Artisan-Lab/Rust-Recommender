// e503
fn main() {
    let mut value = 3;
    let borrow = &mut value;
    let _sum = value + 1; 
    borrow;
}