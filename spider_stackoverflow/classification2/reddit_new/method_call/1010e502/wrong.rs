fn delete(&mut self, item: &T) {
    let index = self.state.get(&item);
    if let Some(count) = index {
        
            self.purge_from_order(&item, count);
            self.purge_from_state(item);
        
    }
}