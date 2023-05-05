fn main() {
    let mut vec1 = vec![4, 5];
    vec1.push(6);
    for i in 0..vec1.len() {
        if vec1[i] % 2 == 0 {
            vec1.push(7);
        }
    }
    vec1.push(8);
    println!("vec1={:?}", vec1);
}