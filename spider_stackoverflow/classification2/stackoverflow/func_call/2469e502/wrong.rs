fn test(a: &mut [i32], place: usize) -> i32 {
    a[place] /= 2;
    return a[place];
}


fn main() {
    let mut values = vec![1, 2, 3, 4];
    let b = test(&mut values, values.len() / 2); // compiler gives error on values.len()
}
// 和之前131问题有点类似 不是调用父亲问题，而是self等交杂使用，不但传一个可变还需要传一个不可变，如果是方法中的self就没事
// 但其实是非self问题