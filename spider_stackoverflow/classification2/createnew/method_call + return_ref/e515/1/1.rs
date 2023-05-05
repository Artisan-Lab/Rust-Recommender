
struct A();
impl A {
    fn mut_ref(&self) -> &i32 {
        let a = 1;
        a;
        return &a;
    }
}
fn main(){
    
}