use std::collections::HashMap;

pub struct SpatialHash {
    pub contents: HashMap<u64, Vec<i32>>,
}

impl SpatialHash {
    pub fn push(&mut self, x: i32, y: i32, entity: i32) {
        let h = SpatialHash::hash(x, y);
        let entities = self.contents.entry(h).or_insert(Vec::new());
        entities.push(entity);
    }

    pub fn get22<'a>(self, x: i32, y: i32) -> Option<&'a Vec<i32>> {
        let h = SpatialHash::hash(x, y);
        self.contents.get(&h)
    }

    // Hash a position into a spatial hash key.
    fn hash(x: i32, y: i32) -> u64 {
        let x = x as u64;
        let y = y as u64;

        x << 32 | y
    }
}