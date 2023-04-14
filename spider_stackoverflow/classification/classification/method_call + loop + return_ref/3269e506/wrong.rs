pub struct Something {
    value: usize,
}

impl Something {
    pub fn get_and_increment(&mut self) -> &[u8] {
        let res = self.get();
        self.value += 1;

        res
    }

    pub fn get(&self) -> &[u8] {
        &[3; 2]
    }
}
fn main()
{
    
}