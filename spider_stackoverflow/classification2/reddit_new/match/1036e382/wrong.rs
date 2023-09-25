 fn find_item_mut(&mut self, names : &Vec<String>) -> 
        Option<&mut MyNode> 
    {
        let tag_len = names.len();

        if tag_len == 0  || self.children.len() == 0 {
            return None;
        }

        let mut r = self.child_mut(&names[0]);

        for i in 0 .. tag_len {            
            match r {
                Some(item) => {
                    if let Some(rc) = item.children.get_mut(&names[i]) {
                        r = Some(rc);
                    }        
                },
                None => break
            }

        }

        return r;

      
    }