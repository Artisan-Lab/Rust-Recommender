struct Element {
    title: String,
}

impl Element {
    pub fn get_title(&self) -> &str {
    &self.title
}
}

fn main() {
    let mut items: Vec<Element> = Vec::new();
    items.push(Element {
        title: "Random".to_string(),
    });
    items.push(Element {
        title: "Gregor".to_string(),
    });

    let mut i = 0;
    while i < 10 {
        for item in &items {
            println!("Loop {} item {}", i, item.get_title());
        }
        i = i + 1;
    }
}