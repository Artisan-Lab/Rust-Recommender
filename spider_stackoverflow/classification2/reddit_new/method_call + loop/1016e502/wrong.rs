struct List {
    items: Vec<Item>,
}

impl List {
    fn update(&mut self) {
        for item in self.items.iter_mut() {
            item.update2(self);
        }
    }

    fn item(&self, pos: usize) -> &Item {
        &self.items[pos]
    }
}

struct Item {
    value: f64,
}

impl Item {
    fn value(&self) -> f64 {
        self.value
    }

    fn update2(&mut self, list: &List) {
        self.value = list.item(1).value() + list.item(2).value();
    }
}
