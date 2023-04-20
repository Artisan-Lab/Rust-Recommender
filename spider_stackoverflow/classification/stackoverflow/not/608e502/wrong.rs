fn main() {
    let mut s = String::from("hello");
    println!("{}", &s);
    let r = &s;
    let x = &mut s;
    println!("{}", r);
    
    println!("{}", x);
}

// 极简问题，只有关于引用的夹杂，都是极简引用和 363 299 本质相同 只是顺序不同