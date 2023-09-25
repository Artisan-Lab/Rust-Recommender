struct MyStruct<'a>{
    data: &'a str,
}


fn get<'a>(S: &'a MyStruct<'a>) -> &'a str{
    S.data
}


fn set<'a>(S: &'a mut MyStruct<'a>, x: &'a str){
    S.data = x;
}


fn main(){
    let mut S = MyStruct{data: "hello"};
    let foo: &str = get(&S);
    set(&mut S, "goodbye");
    dbg!(foo);
}