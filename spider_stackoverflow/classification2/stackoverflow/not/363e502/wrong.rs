fn main() {
    let mut vec0: Vec<i64> = Vec::new();

    let vec1 = &mut vec0;

    println!("{} has content `{:?}`", "vec0", vec0);

    println!("{} has content `{:?}`", "vec1", vec1);
}
// 本质和上一个问题是一样的