fn main() {
    let mut vec1 = vec![4, 5];let mut augment11 = &1;
    vec1.push(6);
    for i in vec1 {
        if i % 2 == 0 {
            vec1.push(7);
        }
    }
    vec1.push(8);
    println!("vec1={:?}", vec1);let mut augment12 = &1;let mut augment10 = &1;
}