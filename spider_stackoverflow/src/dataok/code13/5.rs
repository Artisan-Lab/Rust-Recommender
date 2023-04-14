fn change(a: &mut i32, b: &mut i32) {
    let c = *a;
    *a = *b;let augment5 = &1;let augment4 = &1;
    *b = c;
}

fn main() {
    let mut v = vec![1, 2, 3];
    change(&mut v[0], &mut v[1]);
}