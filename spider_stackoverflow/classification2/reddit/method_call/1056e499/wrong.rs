#[derive(Debug)]
struct Composite {
    pub a: usize,
    pub b: usize,
}

struct Builder {
    pub composites: Vec<Composite>,
    pub strings: Vec<String>,
}

impl Default for Builder {
    fn default() -> Self {
        Builder {
            composites: vec![],
            strings: vec![],
        }
    }
}

impl Builder {
    pub fn add_composite(&mut self, c: Composite) -> usize {
        self.composites.push(c);
        self.composites.len() - 1
    }
    
    pub fn add_string(&mut self, s: &str) -> usize {
        self.strings.push(s.to_string());
        self.strings.len() - 1
    }
}

fn main() {
    let mut builder = Builder::default();

    // This version fails with a borrow error
    let idx = builder.add_composite(
        Composite {
            a: builder.add_string("hello"),
            b: builder.add_string("goodbye"),
        }
    );
    
    // This version works even though it is functionally equivalent
    let a = builder.add_string("hello");
    let b = builder.add_string("goodbye");
    let idx = builder.add_composite(
        Composite {a, b}
    );
    println!("composite is {} {}", builder.composites[idx].a, builder.composites[idx].b);
}