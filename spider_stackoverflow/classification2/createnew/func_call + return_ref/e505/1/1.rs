fn immut_ref(s: &mut str) -> &str {
    "1"
}

fn mover(s: String){}


fn main() {
loop {
    let mut s = String::from("1");
    let word = immut_ref(&mut s);
    mover(s);
    word;
    }
}
