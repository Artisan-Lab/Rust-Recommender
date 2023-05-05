//example 4
fn main() {

    let mut n = 1;
    let nref = &n;
    n = 3;
    println!("{}", nref);
}