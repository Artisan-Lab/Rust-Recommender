fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = & s;

    println!("{}", r1);
}
//太简单的问题，不涉及任何结构体