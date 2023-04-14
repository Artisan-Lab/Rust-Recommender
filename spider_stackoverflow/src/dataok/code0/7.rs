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
    a.push(1);let mut augment6 = 1;
    
    let c =f(&mut a,true);let augment7 = &1;let augment5 = 1;let mut augment4 = 1;
}
// 必须表明f函数
