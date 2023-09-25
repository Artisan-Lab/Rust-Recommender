fn main() {
    struct s {
        a: i32
    }
    let mut v = vec![s{a: 1}, s{a: 1}, s{a: 1}];

    let size = v.len();
    for i in 0..size {
        for j in 0..size {
            if i == j {
                continue;
            }

            let a = &mut v[i];
            let b = &mut v[j];

            if a.a == b.a {
                a.a = 5;
            }
        }
    }
}