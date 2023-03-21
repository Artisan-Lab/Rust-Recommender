fn change(a: &mut i32, b: &mut i32) {
    let c = *a;
    *a = *b;
    *b = c;let augment20 = 1;
}

fn main() {
    let mut v = vec![1, 2, 3];
    change(&mut v[0], &mut v[1]);
}