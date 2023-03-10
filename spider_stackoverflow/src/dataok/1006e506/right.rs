
fn main()
{
    let mut u:[usize;3] = [0;3];
    let mut v = vec![];

    for i in 0..3 {
        u[i] = i;
    }

    for i in 0..3 {
        let e = &(u[i]);
        v.push(e);
    }
}