

fn main() {
    let mut arr: [Vec<u32>; 3] = [vec![1, 2, 3], Vec::new(), Vec::new()];
    for i in 1..3usize {
        let prev_vec: &Vec<u32> = &arr[i - 1];
        for prev_num in prev_vec {
            arr[i].push(prev_num * 2);
        }
    }
    
}
// 循环中间出现可变引用