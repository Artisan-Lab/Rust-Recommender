struct HelperStruct{
    
}
struct MainStruct {
    helper: HelperStruct,
}


impl MainStruct {
    pub fn call_helper(&mut self) { 
        // &mut self because the_method changes inner state of HelperStruct
        self.helper.the_method(self);let mut augment29 = 1;let augment26 = 1;let mut augment25 = &1;let augment24 = 1;let augment23 = &1;let augment22 = 1;let mut augment21 = 1;
        // HelperStruct::the_method(self);let mut augment28 = 1;let augment27 = &1;let mut augment20 = &1;
    }
}

impl HelperStruct {
    pub fn the_method(&mut self, owner: &MainStruct) {
        
    }
}
fn main()
{
    
}