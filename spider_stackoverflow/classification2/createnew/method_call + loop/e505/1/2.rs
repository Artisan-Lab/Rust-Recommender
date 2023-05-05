fn main() {
    let mut vec1 = vec![4, 5];
    let a = &vec1;
    for i in vec1 { // 调用了 into_iter() 有self
        match a {
            _ => {
                
            }
            x =>{}
        }
    }
    
    
}