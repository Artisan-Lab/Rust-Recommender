pub struct Node<'a> {
    _data: i32,
    _next: Option<&'a mut Node<'a>>,
}
impl<'a> Node<'a> {
    pub fn new() -> Self {
        Self {
            _data: 0,
            _next: None,
        }
    }
    pub fn data_mut(&mut self) -> &mut i32 {
        &mut self._data
    }
    pub fn next_mut(&'a mut self) -> &mut Option<&'a mut Self> {
        &mut self._next
    }
}

fn main() {
    let mut head = node::Node::new();
    let mut first = node::Node::new();
    let mut second = node::Node::new();
    *head.data_mut() = 1;
    *first.data_mut() = 2;
    *second.data_mut() = 3;
    *head.next_mut() = Some(&mut first);
    *first.next_mut() = Some(&mut second);
}