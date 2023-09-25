fn main() {
    let boolist: Vec<Boo> = vec![Boo {id: 1, value: 3.2}, Boo {id: 2, value: 7.0}];

    for a in boolist.into_iter() {
        for b in boolist.into_iter() {
            match a {
                b => {},
                _ => {
                    print!("Boo's: {:#?}", add_boos(a, b))
                },
            }
        }
    }
}

#[derive(Debug)]
struct Boo {
    id: u32,
    value: f32,
}

fn add_boos(a: Boo, b: Boo) -> (Boo, Boo) {
    (
        Boo {
            id: a.id,
            value: b.value + a.value,
        },
        
        Boo {
            id: b.id,
            value: a.value - b.value,
        },
    )
}