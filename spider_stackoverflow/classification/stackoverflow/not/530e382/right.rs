fn c(s: &mut String) -> &mut String {
    s
}

fn main() {
    let mut s = String::from("hello");
    let s1 = &mut s;
    let s2 = c(s1);

    *s2 = String::from("world1");
    
    *s1 = String::from("world2");

    println!("{:?}", s);
}