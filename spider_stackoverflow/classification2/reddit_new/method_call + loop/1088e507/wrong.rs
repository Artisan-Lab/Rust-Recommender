pub struct Packet {
    sip: u32,
}

#[derive(Copy,Clone)]
pub struct Rule {
    sip: u32,
}

pub struct RuleBase {
    rules: Vec<Rule>,
}

impl RuleBase {
    fn match_rule(&self, packet : &Packet) {
        for rule in self.rules {
            println!("Hello, world!");
        }
    }
}

fn main() {
    println!("Hello, world!");
}