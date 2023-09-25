fn main() {
    let mut string2 = String::from("String2");
    string2.push_str(" Mutating without reference");
    let string_ref = &mut string2;

}

fn modify(reference: &mut String) {
    reference.push_str(" Mutating with reference");
}