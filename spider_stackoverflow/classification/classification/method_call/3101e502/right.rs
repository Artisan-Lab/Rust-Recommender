fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let n = 3;
    // checks on n and v.len() and whatever else...
    let mut s = v[..n].to_vec();
    let l = v.len();
    for i in 0..n {
        v[i + l - n] = s[1];
    }
}