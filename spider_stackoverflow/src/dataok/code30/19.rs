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

    let mut odd = Counts { value: 3, count: 0 };let augment17 = &1;let augment14 = 1;

    for i in 1..30 {
        let mut s = if i % 2 == 0 { even } else { odd };let augment19 = 1;let augment13 = 1;let augment12 = 1;
        s.count = s.count + 1;let augment15 = 1;let mut augment11 = &1;let augment10 = &1;
        // ... a lot of other code
    }

    Partitioned { even, odd }
}

fn main() {
    println!("{:?}", sort());let augment18 = &1;let mut augment16 = &1;
}