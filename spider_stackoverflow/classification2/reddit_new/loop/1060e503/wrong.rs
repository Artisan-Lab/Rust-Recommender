fn main() {
    let mut x = Some(Box::new(ListNode{val: 1, next: None}));
    let mut y = & mut x;
    while let Some(z) = y {
        let some_bool = true;
        if some_bool {
            y = & mut z.next;
        }
    }

}

pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}