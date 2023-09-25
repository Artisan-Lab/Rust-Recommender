fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = String::from("aaa");
    let l;
    {
        let b = String::from("bbbbbb");
        l = get_longest(&a, &b);
    }
    println!("longest: {:?}", l);
}