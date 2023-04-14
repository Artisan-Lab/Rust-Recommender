enum SomeEnum {
    Foo,
    Bar,
}

struct SomeStruct {
    attrib: SomeEnum,
    next_attrib: Option<SomeEnum>,
}

impl SomeStruct {
    pub fn apply_changes(&mut self) {
        if let Some(se) = self.next_attrib {
            self.attrib = se;
        }
        self.next_attrib = None;
    }
}