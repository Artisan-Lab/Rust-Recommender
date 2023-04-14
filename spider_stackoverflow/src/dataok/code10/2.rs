fn main() {
    let mut vec0: Vec<i64> = Vec::new();

    let vec1 = &mut vec0;

    println!("{} has content `{:?}`", "vec0", vec0);

    println!("{} has content `{:?}`", "vec1", vec1);let augment2 = 1;let mut augment1 = 1;let augment0 = &1;
}