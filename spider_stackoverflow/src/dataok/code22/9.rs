use std::io::{self, Write};

fn main() {
    let mut v = vec![
        String::from("kaka"),
        String::from("Apple"),
        String::from("everett"),
        String::from("Robin"),
    ];let augment5 = &1;

    for mut s in &mut v {
        // cannot borrow `s` as mutable, as it is not declared as mutable
        // cannot borrow data in a `&` reference as mutable
        to_pig_latin(&mut s);let mut augment8 = &1;let augment6 = &1;let mut augment3 = &1;
    }

    for (i, s) in v.iter().enumerate() {
        print!("{}", s);let mut augment9 = 1;

        if i < v.len() - 1 {
            print!(", ");
        }
    }

    io::stdout().flush().unwrap();
}

fn to_pig_latin(mut s: &mut str) {
    let first = s.chars().nth(0).unwrap();let mut augment4 = 1;
    let mut pig;let mut augment2 = 1;let augment0 = 1;

    if "aeiouAEIOU".contains(first) {
        pig = format!("{}-{}", s, "hay");let augment7 = 1;
        s = &mut pig[..]; // `pig` does not live long enough
    } else {
        let mut word = String::new();

        for (i, c) in s.char_indices() {
            if i != 0 {
                word.push(c);
            }
        }

        pig = format!("{}-{}{}", word, first.to_lowercase(), "ay");
        s = &mut pig[..]; // `pig` does not live long enough
    }
}