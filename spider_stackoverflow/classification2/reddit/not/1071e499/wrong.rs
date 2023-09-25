fn main() {

    let mut v = vec![];

    let size = 1;
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