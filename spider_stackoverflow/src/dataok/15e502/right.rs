// fn f(v: &mut Vec<i32>, a: bool) -> &i32 {
//     match &v[0] {
//         _ if a => {
//             f(v, !a);
//             &v[0]
//         }
//         x => {
//             x
//         }
//     }
// }

fn f(v: &mut Vec<i32>, a: bool) -> &i32 {
    match v[0] {
        _ if a => {
            f(v, !a);
            &v[0]
        }
        _ => &v[0],
    }
}

fn main()
{
    let mut a = Vec::<i32>::new();
    a.push(1);
    
    let c =f(&mut a,true);
}