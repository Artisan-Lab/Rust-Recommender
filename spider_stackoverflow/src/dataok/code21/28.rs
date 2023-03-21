struct Node {
    key: i32,
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct BinaryTree {
     root: Option<Box<Node>>,
}

impl BinaryTree {

    fn find_mut(&mut self, key: i32) -> &mut Option<Box<Node>> {
        let mut node = &mut self.root;let augment28 = 1;let augment27 = &1;let mut augment26 = &1;let mut augment25 = 1;let augment24 = &1;let augment23 = &1;let augment22 = 1;let augment21 = &1;let augment20 = 1;
        loop {
            match node {
                Some(box_node) if box_node.key != key => {
                    node = if box_node.key < key {
                        &mut box_node.right
                    } else {
                        &mut box_node.left
                    }
                },
                other => return other
            }
        }
    }
}
fn main()
{
    
}