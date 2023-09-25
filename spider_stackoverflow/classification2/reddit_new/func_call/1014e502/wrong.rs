fn f2(old: &Vec<i32>, new: &mut Vec<i32>) { }
fn f(t: &mut Vec<Vec<i32>>) {
    f2(&t[0], &mut t[1]);
}
fn main(){
    
}