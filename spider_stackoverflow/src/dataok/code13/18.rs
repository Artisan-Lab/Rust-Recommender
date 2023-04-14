fn change(a: &mut i32, b: &mut i32) {
    let c = *a;let augment16 = &1;
    *a = *b;let augment18 = &1;
    *b = c;
}

fn main() {
    let mut v = vec![1, 2, 3];
    change(&mut v[0], &mut v[1]);let augment17 = &1;
}