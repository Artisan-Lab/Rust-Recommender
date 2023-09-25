use std::cell::Cell;

pub struct Node<T> {
    pub data: T,
    next: Cell<Option<Box<Node<T>>>>
}


impl<T> Node<T> {
    fn set_next(self, next: Node<T>){
        self.next.set(Some(Box::new(next)));
    }

    fn null_next(self) {
        self.next.set(None);
    }

    fn get_next(self) -> Option<Box<Node<T>>> {
        return match self.next.into_inner() {
            None => None,
            Some(ptr) => Some(ptr),
        }
    }

    fn new(data: T) -> Node<T> {
        Node {
            data,
            next: Cell::new(None)
        }
    }
}


fn test_node_linking() {
        let n1: Node<i32> = Node::new(10);
        let n2: Node<i32> = Node::new(20);
        n1.set_next(n2);

        assert_eq!(n1.data, 10);
        assert_eq!(n1.next.into_inner().is_none(), false);

        assert_eq!(n1.get_next().unwrap().data, 20);
        assert_eq!(n1.get_next().unwrap().next.into_inner().is_none(), true);
    }