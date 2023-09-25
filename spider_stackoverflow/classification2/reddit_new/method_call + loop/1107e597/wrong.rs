use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
type TreeNodeRef = Rc<RefCell<TreeNode>>;
#[derive(Debug, Clone)]
pub struct TreeNode {
    val: i32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

///        -1
///      /    \
///    -6     -5
///   /  \      \
/// -3   -4    -13
///     / \    /    
///    -2  6  7
/// Then the right-most value in the bottom-most
/// level is 7
pub fn bottom_right_value(root: TreeNodeRef) -> i32 {
    // `Rc.clone()` is cheap so use
    let mut current = root.clone();
    let mut queue: VecDeque<TreeNodeRef> = VecDeque::new();
    queue.push_back(root);
    while !queue.is_empty() {
        current = queue.pop_front().unwrap();

        if let Some(left) = &current.borrow().left {
            queue.push_back(left.clone());
        };
        if let Some(right) = &current.borrow().right {
            queue.push_back(right.clone());
        };
    }
   
    current.borrow().val
}