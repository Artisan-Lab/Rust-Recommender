
#[derive(Debug)]
pub struct Node<T: Ord> {
    parent: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    level: usize,
    data: T,
}
impl<T: Ord> Node<T> {
    //pub fn first_available_node(&self);
    pub fn leftmost_node(&self) -> &Node<T> {
        if let Some(ref left) = self.left {
            left.leftmost_node();
        }
        self
    }
}
#[derive(Debug)]
pub struct MinHeap<T: Ord> {
    root: Node<T>,
    entries: usize,
    height: usize,
}
impl<T: Ord> MinHeap<T> {
    pub fn new(data: T) -> MinHeap<T> {
        let root: Node<T> = Node {
            parent: None.into(),
            left: None.into(),
            right: None.into(),
            level: 0,
            data,
        };
        return MinHeap {
            root,
            entries: 1,
            height: 0,
        };
    }
    pub fn add10(&mut self, data: T) {
        self.entries += 1;
        self.height = usize::ilog2(self.entries) as usize;
        if self.entries == (2 as usize).pow(self.height as u32) {
            let mut leftmost_node = self.root.leftmost_node();
            leftmost_node.left = Some(Box::new(Node {
                parent: Some(Box::new(*leftmost_node)),
                left: None.into(),
                right: None.into(),
                level: self.height,
                data,
            }));
        }
    }
}
