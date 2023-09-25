#[derive(Debug,Clone,Copy)]
struct Num(pub u32);

impl std::ops::AddAssign<Num> for Num {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

fn main() {
    let mut vec: Vec<u32> = vec![0, 1];
    vec[0] += vec[1];

    let mut numvec: Vec<Num> = vec![Num(0), Num(1)];
    numvec[0] += numvec[1];
}