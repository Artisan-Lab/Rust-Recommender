fn change(a: &mut i32, b: &mut i32) {
    let c = *a;let mut augment23 = &1;let augment22 = &1;
    *a = *b;
    *b = c;let mut augment21 = &1;let augment20 = 1;
}

fn main() {
    let mut v = vec![1, 2, 3];
    change(&mut v[0], &mut v[1]);
}