pub struct Person {
    pub name: String,
}

fn main() {
    let mut ppl: Vec<Person> = vec![];
    ppl.push(Person{name: "Joe".to_owned()});
    ppl.push(Person{name: "Mike".to_owned()});
    
    let mut ref0 = &mut ppl[0];
    let mut ref1 = &mut ppl[1];
    
    ref0.name = "AA".to_owned();

}