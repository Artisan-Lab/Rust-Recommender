use std::collections::BTreeMap;

struct Tree {
    map: BTreeMap<String, Tree>,
}

impl Tree {
    fn mk_path(&mut self, path: &[String]) {
        let mut current = self;
        for p in path {
            if let Some(c) = current.map.get_mut(p) {
                current = c;
                continue;
            }
            current.map.insert(
                p.to_string(),
                Tree {
                    map: BTreeMap::new(),
                },
            );
            current = current.map.get_mut(p).unwrap();
        }
    }
}