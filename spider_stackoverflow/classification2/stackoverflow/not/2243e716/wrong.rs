#[derive(Debug)]
struct Test {
    val: i32
}

fn main() {
    let a:&Test;
    let b = 32;

    a = &Test {
        val: b
    };

    println!("{:?}", a.val);
}