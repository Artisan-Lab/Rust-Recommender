fn main() {
    let mut vec1 = vec![4, 5];let augment9 = 1;let mut augment8 = 1;
    vec1.push(6);let augment10 = &1;
    for i in vec1 {
        if i % 2 == 0 {
            vec1.push(7);
        }
    }
    vec1.push(8);
    println!("vec1={:?}", vec1);
}