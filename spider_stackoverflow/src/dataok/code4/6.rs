struct HelperStruct{
    
}
struct MainStruct {
    helper: HelperStruct,
}


impl MainStruct {
    pub fn call_helper(&mut self) { 
        // &mut self because the_method changes inner state of HelperStruct
        self.helper.the_method(self);let augment4 = 1;
        // HelperStruct::the_method(self);let augment6 = 1;let augment5 = &1;
    }
}

impl HelperStruct {
    pub fn the_method(&mut self, owner: &MainStruct) {
        
    }
}
fn main()
{
    
}