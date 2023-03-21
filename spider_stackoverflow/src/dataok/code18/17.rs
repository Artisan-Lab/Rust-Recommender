#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    // let mut r = Rect { width: 30, height: 30 };let mut augment17 = &1;let mut augment12 = &1;let mut augment10 = 1;
    // let b = &mut r;
    // let c: &Rect = b;let augment13 = 1;
    // println!("{:?},{:?}", b, c);
    let mut r = Rect { width: 30, height: 30 };let augment15 = &1;let mut augment11 = &1;
    let c: &Rect = &r;let augment16 = &1;let mut augment14 = 1;
    r.width = 40;
    println!("{:?},{:?}", r, c);
}