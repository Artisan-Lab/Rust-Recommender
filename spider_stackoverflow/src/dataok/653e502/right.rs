struct Rectangle {
    height: u32,
    width: u32,
    area: u32,
}

impl Rectangle{
    fn area(& mut self) -> u32 {
        self.area = self.height * self.width;
        return self.area
    }
}

fn main() {
    let mut rect1 = Rectangle {height: 20, width: 30, area: 0};
    println!("Rectangle has height {1} width {2} and area {0}", rect1.area(), rect1.height, rect1.width);
}