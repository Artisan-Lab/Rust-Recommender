// e382
// 与 7 一样，但是出现错误位置不同
struct x {}

fn cal(number: x){
    
}

fn main()
{
    let mut a = x{};
    for i in 0..10 {
        let b = &a;
        cal(a);

    }
}