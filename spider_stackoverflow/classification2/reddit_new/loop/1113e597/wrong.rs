pub fn iter_insert(node: &mut Rc<RefCell<Node<T>>>, value: T) {
    let mut tmp = node;
    let mut selected;
    let _selected_node = {
      loop {
        let mut borrow = tmp.borrow_mut();
        selected = match borrow.value.cmp(&value) {
          Less => &mut borrow.right,
          Greater => &mut borrow.left,
          Equal => break None,
        };
  
        if let Some(ref mut child) = selected {
          tmp = child;
        } else {
          break Some(&tmp);
        }
      }
    };
  }