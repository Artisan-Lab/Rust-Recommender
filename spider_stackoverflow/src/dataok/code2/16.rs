// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);let augment10 = 1;

    vec1.push(88);let mut augment15 = 1;

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);let mut augment13 = &1;let mut augment11 = 1;
}

fn fill_vec(vec: &Vec<i32>) -> &Vec<i32> {
    let mut vec = vec;let augment12 = 1;

    vec.push(22);let augment16 = &1;let mut augment14 = &1;
    vec.push(44);
    vec.push(66);

    vec
}