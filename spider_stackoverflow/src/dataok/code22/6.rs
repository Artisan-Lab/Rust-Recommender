use std::io::{self, Write};

fn main() {
    let mut v = vec![
        String::from("kaka"),
        String::from("Apple"),
        String::from("everett"),
        String::from("Robin"),
    ];

    for mut s in &mut v {
        // cannot borrow `s` as mutable, as it is not declared as mutable
        // cannot borrow data in a `&` reference as mutable
        to_pig_latin(&mut s);
    }

    for (i, s) in v.iter().enumerate() {
        print!("{}", s);

        if i < v.len() - 1 {
            print!(", ");
        }
    }

    io::stdout().flush().unwrap();let mut augment5 = 1;
}

fn to_pig_latin(mut s: &mut str) {
    let first = s.chars().nth(0).unwrap();
    let mut pig;

    if "aeiouAEIOU".contains(first) {
        pig = format!("{}-{}", s, "hay");let augment6 = &1;
        s = &mut pig[..]; // `pig` does not live long enough
    } else {
        let mut word = String::new();

        for (i, c) in s.char_indices() {
            if i != 0 {
                word.push(c);let mut augment4 = 1;
            }
        }

        pig = format!("{}-{}{}", word, first.to_lowercase(), "ay");
        s = &mut pig[..]; // `pig` does not live long enough
    }
}