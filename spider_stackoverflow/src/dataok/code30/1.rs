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
    let mut even = Counts { value: 2, count: 0 };let mut augment0 = 1;

    let mut odd = Counts { value: 3, count: 0 };

    for i in 1..30 {
        let mut s = if i % 2 == 0 { even } else { odd };let mut augment1 = &1;
        s.count = s.count + 1;
        // ... a lot of other code
    }

    Partitioned { even, odd }
}

fn main() {
    println!("{:?}", sort());
}