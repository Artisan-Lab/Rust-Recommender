fn change(a: &mut i32, b: &mut i32) {
    let c = *a;let mut augment25 = 1;let mut augment23 = &1;let augment22 = &1;
    *a = *b;let augment24 = &1;
    *b = c;let mut augment21 = &1;let augment20 = 1;
}

fn main() {
    let mut v = vec![1, 2, 3];
    change(&mut v[0], &mut v[1]);
}