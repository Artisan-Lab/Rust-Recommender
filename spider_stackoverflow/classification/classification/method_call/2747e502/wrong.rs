struct SomeStruct {
    val: i32
}

fn bor_mut(bor: &mut SomeStruct, new_val: i32) -> &i32 {
    bor.val = new_val;
    &bor.val
}

fn bor_nmut(bor: &SomeStruct) -> &i32 {
    &bor.val
}

fn main() {
    // get two non-mutable borrows,
    // but one originates from a mutable
    // borrow
    let mut s = SomeStruct{val: 1};
    let a = bor_mut(&mut s, 2);
    let b = bor_nmut(&s);
    println!("a = {}, b = {}", a, b);
}
// 有类似问题 返回的&i32与&mut s 同生命周期了