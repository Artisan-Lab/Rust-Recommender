fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let n = 3;
    // checks on n and v.len() and whatever else...
    let mut s = v[..n].to_vec();
    for i in 0..n {
        v[i + v.len() - n] = s[1];
    }
}
// 里面ref的使用 比较简单