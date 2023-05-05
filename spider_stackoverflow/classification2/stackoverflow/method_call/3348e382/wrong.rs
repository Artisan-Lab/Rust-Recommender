#[derive(Debug)]
struct Node {
    children: Vec<Node>,
}

impl Node {
    fn new(children: Vec<Self>) -> Self {
        Self { children }
    }
    fn get_last(&mut self) -> Option<&mut Node> {
        self.children.last_mut()
    }
}

fn main() {
    let mut root = Node::new(vec![Node::new(vec![])]);

    let current = &mut root;

    println!("Final: {:?}", get_last(current));
}


fn get_last(mut current: &mut Node) -> &mut Node {
    loop {
        let temp = current;
        println!("{:?}", temp);

        match temp.get_last() {
            Some(child) => { current = child },
            None => break,
        }
    }

    current
}