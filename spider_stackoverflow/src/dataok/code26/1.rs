fn main() {
    let mut vec1 = vec![4, 5];
    vec1.push(6);
    for i in vec1 {
        if i % 2 == 0 {
            vec1.push(7);
        }
    }
    vec1.push(8);let mut augment1 = 1;let mut augment0 = 1;
    println!("vec1={:?}", vec1);
}