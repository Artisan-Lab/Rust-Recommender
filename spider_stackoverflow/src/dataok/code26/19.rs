fn main() {
    let mut vec1 = vec![4, 5];let mut augment19 = &1;let mut augment17 = &1;
    vec1.push(6);
    for i in vec1 {
        if i % 2 == 0 {
            vec1.push(7);let augment18 = 1;
        }
    }
    vec1.push(8);let augment16 = 1;
    println!("vec1={:?}", vec1);
}