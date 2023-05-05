fn immut_ref(s: &str) -> &str {
    "1"
}

fn main() {
    let mut s = String::from("1");
    let word = immut_ref(&s);
    s.clear();
    loop {
    match word {
        x => {}
    }
    }
    
}
