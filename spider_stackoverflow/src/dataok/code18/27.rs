#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    // let mut r = Rect { width: 30, height: 30 };let augment24 = 1;
    // let b = &mut r;
    // let c: &Rect = b;let mut augment27 = &1;
    // println!("{:?},{:?}", b, c);let mut augment25 = 1;let augment21 = 1;
    let mut r = Rect { width: 30, height: 30 };let mut augment26 = &1;let augment23 = &1;
    let c: &Rect = &r;
    r.width = 40;let augment20 = &1;
    println!("{:?},{:?}", r, c);let augment22 = 1;
}