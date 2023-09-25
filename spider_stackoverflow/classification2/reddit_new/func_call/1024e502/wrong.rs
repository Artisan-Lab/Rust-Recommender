
fn i32_merge_sort(a: &mut [i32]){
    
}
fn main()
{
    
    let mut array = Vec::new();


    i32_merge_sort(&mut array[0 .. array.len() / 2]);
    i32_merge_sort(&mut array[array.len() / 2.. array.len()]);
}