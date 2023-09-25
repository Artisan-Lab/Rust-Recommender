#[derive(Debug)]
struct Test<'a> {
    t: &'a i32,
    v: Vec<i32>
}

fn main() {
    let v = vec![1, 2];
    let t = Test {
        t: &v[0],
        v: v
    };

    println!("{:?}", t);
}