use rand::prelude::*;

#[derive(Clone, Debug)]
struct Node {
    val: String,
    next: Option<Box<Node>>,
}

fn append(mut spot: &mut Option<Box<Node>>, val: String) {
    // Find the tail of the list.
    while let Some(node) = spot {
        spot = &mut node.next;
    }
    // This compiles just fine.
    *spot = Some(Box::new(Node { val, next: None }));
}

fn append_or_maybe_overwrite(mut spot: &mut Option<Box<Node>>, val: String) {
    // Find the tail of the list, or maybe stop at an earlier node.
    while let Some(node) = spot {
        if rand::thread_rng().gen_range(0..10) == 0 {
            break;
        }
        spot = &mut node.next;
    }
    // This fails to compile with:
    // error[E0506]: cannot assign to `*spot` because it is borrowed
    *spot = Some(Box::new(Node { val, next: None }));
}

fn main() {
    // Just a quick example of what append() is doing.
    let mut list = None;
    append(&mut list, "foo".into());
    append(&mut list, "bar".into());
    dbg!(list); // "foo" -> "bar" -> None
}