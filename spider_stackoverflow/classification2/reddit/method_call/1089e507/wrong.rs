use num_rational::*;

#[derive(Clone)]
struct Coords {
    x: BigRational,
    y: BigRational,
    z: BigRational
}

impl Coords {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}