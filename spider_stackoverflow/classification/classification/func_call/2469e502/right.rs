fn test(a: &mut [i32], place: usize) -> i32 {
    a[place] /= 2;
    return a[place];
}


fn main() {
    let mut values = vec![1, 2, 3, 4];
    let l  =values.len() / 2;
    let b = test(&mut values, l); // compiler gives error on values.len()

}
