#[derive(Debug)]
struct Node {
    valor: Option<String>,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(valor: Option<String>) -> Node {
        Node { valor, next: None }
    }
}

#[derive(Debug)]
struct NodeList {
    head: Option<Box<Node>>,
}
impl NodeList {
    fn new() -> NodeList {
        NodeList { head: None }
    }
    fn append(&mut self, novo: Node) {
        if self.head.is_none() {
            self.head = Some(Box::new(novo));
        } else {
            let mut current: &mut Node = &mut self.head.as_mut().unwrap();
            loop {
                if current.next.is_none() {
                    current.next = Some(Box::new(novo));
                    break;
                }
                current = current.next.as_mut().unwrap().as_mut();
            }
        }
    }
}

fn main() {
  
  }