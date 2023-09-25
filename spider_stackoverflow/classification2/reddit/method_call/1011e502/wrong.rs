fn iter_mut(&mut self){}
fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    let slice: &[i32] = &numbers[1..3];
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    slice;
}
fn main()
{
    
}