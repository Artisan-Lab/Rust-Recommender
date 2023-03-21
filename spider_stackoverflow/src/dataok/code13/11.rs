fn change(a: &mut i32, b: &mut i32) {
    let c = *a;let augment10 = 1;
    *a = *b;let mut augment11 = &1;
    *b = c;
}

fn main() {
    let mut v = vec![1, 2, 3];
    change(&mut v[0], &mut v[1]);
}