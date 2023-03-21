// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let vec0 = Vec::new();let augment21 = &1;

    let mut vec1 = fill_vec(&vec0);let augment25 = &1;

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);let augment27 = &1;

    vec1.push(88);let mut augment26 = 1;let augment23 = &1;

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> &Vec<i32> {
    let mut vec = vec;

    vec.push(22);let mut augment24 = &1;let mut augment20 = &1;
    vec.push(44);
    vec.push(66);let mut augment22 = 1;

    vec
}