struct Foo {
    my_first_string: String,
}

struct Bar {
    my_second_string: String,
    string_is_empty: bool,
}

impl From<Foo> for Bar {
    fn from(item: Foo) -> Self {
        Bar {
            my_second_string: item.my_first_string,
            string_is_empty: item.my_first_string.is_empty(),
        }
    }
}
fn main(){
    
}