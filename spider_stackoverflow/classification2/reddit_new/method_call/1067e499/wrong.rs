fn main() {
    let mut a = 1;
    let b = &mut a;
    let c = &mut a;
    //println!("{:?}", a); // WORKS
    println!("{:?}", b); // DOESN'T WORK
    //println!("{:?}", c); // WORKS
}