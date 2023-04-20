#[derive(Debug)]
struct Test {
    val: i32
}

fn main() {
    let a:&Test;
    let b = 32;

    let a = &Test {
        val: b
    };

    println!("{:?}", a.val);
}