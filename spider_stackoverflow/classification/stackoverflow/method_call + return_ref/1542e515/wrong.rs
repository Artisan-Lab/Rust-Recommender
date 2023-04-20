struct SomeData(u8, u8);

impl SomeData {
    fn to_bytes(&self) -> &[u8] {
        let mut bytes: [u8; 16] = [0; 16];

        // fill up buffer with some data in `SomeData`.
        bytes[0] = self.0;
        bytes[1] = self.1;

        // return slice
        &bytes[..]
    }
}
fn main(){
    
}