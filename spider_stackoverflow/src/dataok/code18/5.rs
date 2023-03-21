#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    // let mut r = Rect { width: 30, height: 30 };let mut augment5 = &1;let augment4 = &1;let augment1 = &1;
    // let b = &mut r;
    // let c: &Rect = b;let mut augment0 = &1;
    // println!("{:?},{:?}", b, c);
    let mut r = Rect { width: 30, height: 30 };let augment3 = 1;
    let c: &Rect = &r;let mut augment2 = 1;
    r.width = 40;
    println!("{:?},{:?}", r, c);
}