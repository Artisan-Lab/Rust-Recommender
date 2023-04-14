fn f(v: &mut Vec<i32>, a: bool) -> &i32 {
    match &v[0] {
        _ if a => {
            f(v, !a);let mut augment0 = &1;
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
    a.push(1);let augment3 = &1;let mut augment1 = 1;
    
    let c =f(&mut a,true);let mut augment2 = 1;
}
// 必须表明f函数
