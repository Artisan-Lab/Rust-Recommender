fn get2(self: &mut Self, key: &String) -> &T
{
    {
        let value = self.listing.get(key);
        match value
        {
            Some(v) => { return v; },
            None => {}
        };
    }
    self.listing.insert(key.clone(), T::new(&key));
    return self.listing.get(key).unwrap();
}
