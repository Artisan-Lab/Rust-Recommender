#[derive(Debug)]    
struct Point {    
    x: usize,    
    y: usize,    
}    
#[derive(Debug)]    
struct Test {    
    point_a: Point,    
    point_b: Point,    
}    
    
impl Test {    
    pub fn get_point_a_mut(&mut self) -> &mut Point {    
        return &mut self.point_a;
    }    
}

fn main() {    
    let mut t = Test {    
        point_a: Point { x: 0, y: 0 },    
        point_b: Point { x: 1, y: 1 },    
    };    
    
    let a = t.get_point_a_mut(); // <-- changed here
    let b = &mut t.point_b;    
    
    println!["{:?}\n{:?}", a, b];    
}