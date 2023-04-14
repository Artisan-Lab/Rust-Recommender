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
    println!("Rectangle has height {} width {} and area {}", rect1.height, rect1.width, rect1.area());
}

// 与println宏密切相关， println 自带了对rect1.height的引用，这里注意问题, 至于为什么area() 放在前面就可以了，因为area()在使用self之后返回的是u32而不是引用
// 看一下println 宏内部作用域