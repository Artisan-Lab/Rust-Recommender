struct A {
    a: u32,
}

impl A{
    fn mut_ref(&mut self) -> u32 {
        return self.a
    }
}

fn main() {
    let mut b = A{a:1};
    println!("{} {}",b.a,b.mut_ref());

}

// 与println宏密切相关