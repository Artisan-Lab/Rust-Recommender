struct Tree {
    root: Option<Box<Node>>
}

#[derive(PartialEq)]
struct Node {
    val: u128,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

pub fn insert(&mut self, val: u128) {
    let mut new_node: Node = Node {
        val,
        left: None,
        right: None
    };

    let mut curr_op: &mut Option<Box<Node>> = &mut self.root;

    loop {
        if let Some(ref mut curr_node) = curr_op {
            if val == curr_node.val {
                // This is the key line where I attempt the swaps
                new_node.left = mem::replace(&mut curr_node.left,                         
                                         Some(Box::new(new_node))
                                );
                return;
            } else if val < curr_node.val {
                curr_op = &mut curr_node.left;

            } else {
                curr_op = &mut curr_node.right;
            }

        } else { // curr_op is None
            curr_op = &mut Some(Box::new(new_node));
            return;
        }
    }
}