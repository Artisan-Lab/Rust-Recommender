fn change(a: &mut i32, b: &mut i32) {
    let c = *a;let augment10 = 1;
    *a = *b;let mut augment11 = &1;
    *b = c;let mut augment15 = 1;
}

fn main() {
    let mut v = vec![1, 2, 3];let augment13 = &1;let mut augment12 = 1;
    change(&mut v[0], &mut v[1]);let augment14 = &1;
}