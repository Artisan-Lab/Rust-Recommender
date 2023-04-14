use std::mem;

struct Node{
    elem: i32,
    next : Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List{
    head: Link,
}

impl List{
    pub fn pop_node(&mut self) -> Link{
        let node = mem::replace(&mut self.head, Link::Empty);
        match node {
            Link::More(nd) => {self.head= nd.next;}
            _ => ()
        };
        node
    }
}
fn main()
{
    
}