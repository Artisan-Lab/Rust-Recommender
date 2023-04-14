fn main() {
    let mut vec0: Vec<i64> = Vec::new();
    let vec1 = & vec0;
    println!("{} has content `{:?}`", "vec0", vec0);
    
    
    println!("{} has content `{:?}`", "vec1", vec1);
}