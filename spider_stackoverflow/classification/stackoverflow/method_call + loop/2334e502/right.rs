fn main() {
    let mut arr: [Vec<u32>; 3] = [vec![1, 2, 3], Vec::new(), Vec::new()];
    for i in 1..3usize {
        let (prev,curr) = arr.split_at_mut(i);
        let prev_arr = prev.last().unwrap();
        let this_arr = curr.first_mut().unwrap();
        for prev_num in prev_arr {
            this_arr.push(prev_num * 2);
        }
    }
    dbg!(arr);
}