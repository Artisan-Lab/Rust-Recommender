fn change(a: &mut i32, b: &mut i32) {
    let c = *a;
    *a = *b;let mut augment0 = 1;
    *b = c;let mut augment1 = &1;
}

fn main() {
    let mut v = vec![1, 2, 3];
    change(&mut v[0], &mut v[1]);let augment2 = 1;
}