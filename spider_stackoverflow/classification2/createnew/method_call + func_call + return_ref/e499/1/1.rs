fn immut_ref(s: &mut str) -> &str {
    "1"
}

fn main() {
    let mut s = String::from("1");
    let word = immut_ref(&mut s);
    s.clear();
    word;
    word;
}
