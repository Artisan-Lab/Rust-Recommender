fn change(a: &mut i32, b: &mut i32) {
    let c = *a;
    *a = *b;
    *b = c;
}

fn main() {
    let mut v = vec![1, 2, 3];let mut augment1 = &1;
    change(&mut v[0], &mut v[1]);let mut augment0 = 1;
}