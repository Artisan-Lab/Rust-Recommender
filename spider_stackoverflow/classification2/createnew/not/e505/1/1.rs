struct MyStruct { s: u32 }

fn main() {
    loop {
    let mut x = MyStruct{ s: 5u32 };
    let a = &x;
    let y = x;
    a;
    }
    
}
// 与另外一个的区别主要在于返回的error信息不太一样
// error[E0382]: assign to part of moved value: `x`