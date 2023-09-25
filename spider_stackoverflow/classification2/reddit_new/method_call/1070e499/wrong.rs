struct Particle { 
    x: [i32;3],
    id: usize,
}

impl Particle {
    pub fn new(x:[i32;3],id:usize) -> Self { 
        Self { 
            x,
            id,
        }
    }

    fn get_position(&self) -> [i32;3] {
        self.x 
    }

    pub fn lets_move(&mut self,particles:&mut Vec<Particle>) {
        for i in 0..particles.len() {
            if i != self.id {
                let x = particles[i].get_position();
                for coord in 0..3 {
                    self.x[coord] += x[coord];
                }
            }
        }
    }

}

fn main() {
    let mut a = Vec::<Particle>::new();
    a.push(Particle::new([1,2,3],0));
    a.push(Particle::new([1,5,8],1));
    a[0].lets_move(&mut a);

} 