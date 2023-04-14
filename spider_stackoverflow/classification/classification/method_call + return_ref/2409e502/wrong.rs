#[derive(Debug)]
struct Foo(i32);

impl Foo {
    fn doo(&mut self) -> &Self {
        self.0 += 1;
        self // implicit coercion
    }
}

fn main() {
    let mut foo = Foo(0);
    let bar = foo.doo();
    println!("{:?}, {:?}", &foo, bar);
}
// 生命周期扩展 只要bar活着 foo就被认为是可变引用 因为在doo方法里调用了&mut self