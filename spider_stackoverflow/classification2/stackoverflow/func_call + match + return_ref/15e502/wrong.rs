fn f(v: &mut Vec<i32>, a: bool) -> &i32 {
    match &v[0] {
        _ if a => {
            f(v, !a);
            &v[0]
        }
        x => {
            x
        }
    }
}
fn main()
{
    let mut a = Vec::<i32>::new();
    a.push(1);
    
    let c =f(&mut a,true);
}
// 必须表明f函数

// 这里的问题是 如果是x，不会copy，则x生命周期和v一样 

