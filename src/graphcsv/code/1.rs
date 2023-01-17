// e382
fn main()
{
    let a = vec![1];
    let b = max(a);
    let c = min(a);
}
fn max(array: Vec<i32>) -> i32 {
    let mut max = array[0];
    for val in array {
        if max < val {
            max = val;
        }
    }
    max
}

fn min(array: Vec<i32>) -> i32{
    let mut min = array[0];
    for val in array {
        if min > val {
            min = val;
        }
    }
    min
}
