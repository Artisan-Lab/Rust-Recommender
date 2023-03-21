fn f(v: &mut Vec<i32>, a: bool) -> &i32 {
    match &v[0] {
        _ if a => {
            f(v, !a);let augment16 = 1;let augment15 = &1;let augment11 = &1;
            &v[0]
        }
        x => {
            x
        }
    }
}
fn main()
{
    let mut a = Vec::<i32>::new();let mut augment17 = &1;let augment13 = 1;
    a.push(1);let augment14 = 1;let mut augment12 = &1;
    
    let c =f(&mut a,true);let augment10 = 1;
}
// 必须表明f函数
