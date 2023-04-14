#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    // let mut r = Rect { width: 30, height: 30 };
    // let b = &mut r;let mut augment14 = &1;let mut augment13 = 1;
    // let c: &Rect = b;
    // println!("{:?},{:?}", b, c);
    let mut r = Rect { width: 30, height: 30 };
    let c: &Rect = &r;
    r.width = 40;let mut augment12 = 1;
    println!("{:?},{:?}", r, c);
}