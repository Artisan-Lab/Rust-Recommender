#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let mut r = Rect { width: 30, height: 30 };
    let b = &mut r;
    let c: &Rect = b;
    println!("{:?},{:?}", b, c);

}