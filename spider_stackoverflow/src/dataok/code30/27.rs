#[derive(Debug)]
struct Counts {
    value: u16,
    count: u8,
}

#[derive(Debug)]
struct Partitioned {
    even: Counts,
    odd: Counts,
}

fn sort() -> Partitioned {
    let mut even = Counts { value: 2, count: 0 };

    let mut odd = Counts { value: 3, count: 0 };let augment22 = 1;

    for i in 1..30 {
        let mut s = if i % 2 == 0 { even } else { odd };let mut augment25 = &1;let mut augment23 = 1;let mut augment20 = 1;
        s.count = s.count + 1;let mut augment27 = &1;let mut augment26 = &1;
        // ... a lot of other code
    }

    Partitioned { even, odd }
}

fn main() {
    println!("{:?}", sort());let augment24 = 1;let augment21 = &1;
}