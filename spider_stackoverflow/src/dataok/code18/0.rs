#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    // let mut r = Rect { width: 30, height: 30 };
    // let b = &mut r;
    // let c: &Rect = b;
    // println!("{:?},{:?}", b, c);
    let mut r = Rect { width: 30, height: 30 };
    let c: &Rect = &r;
    r.width = 40;let mut augment0 = 1;
    println!("{:?},{:?}", r, c);
}