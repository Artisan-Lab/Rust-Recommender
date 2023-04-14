struct Node {
    valor: Option<String>,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(valor: Option<String>) -> Node {
        Node { valor, next: None }
    }
}

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
            let mut current: &mut Option<Box<Node>> = &mut self.head;
            loop {
                let after: &mut Option<Box<Node>> = &mut (current.as_mut().unwrap()).next;
                if after.is_none() {
                    break;
                } else {
                    current = after;
                }
            }
            current.as_mut().unwrap().next = Some(Box::new(novo));
        }
    }
}

fn main() {
    let mut lista = NodeList::new();

    for c in "açafrão".chars() {
        let s = String::from(c);
        lista.append(Node::new(Some(s)));
    }
}