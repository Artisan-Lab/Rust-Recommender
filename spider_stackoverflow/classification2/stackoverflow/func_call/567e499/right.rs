// fn change(a: &mut i32, b: &mut i32) {
//     let c = *a;
//     *a = *b;
//     *b = c;
// }

// fn main() {
//     let mut v = vec![1, 2, 3];
//     change(&mut v[0], &mut v[1]);
// }


fn change(a: &mut i32, b: &mut i32) {
    let c = *a;
    *a = *b;
    *b = c;
}

fn main() {
    let mut arr = [5, 6, 7, 8];
    {
        let [ref mut a, _, ref mut b, ..] = arr;
        change(a, b);
    }
    assert_eq!(arr, [7, 6, 5, 8]);
}