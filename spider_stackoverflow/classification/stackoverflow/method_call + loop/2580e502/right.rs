struct Package {
    id: u32,
}

impl Package {
    fn new(id: u32) -> Package {
        Package { id }
    }
}

struct Manager {
    packages: Vec<Package>,
}

impl Manager {
    fn new() -> Manager {
        Manager {
            packages: vec![
                Package::new(1),
                Package::new(2),
                Package::new(3),
                Package::new(4),
            ],
        }
    }
    fn run(&mut self) {
    for package_idx in 0..self.packages.len() {
        if self.packages[package_idx].id == 1 {
            self.send(package_idx);
        }
        println!("{}", self.packages[package_idx].id);
    }
}

fn send(&mut self, package_idx: usize) {
    self.packages[package_idx].id = 23;
}
}

fn main() {
    let mut manager = Manager::new();
    manager.run();
}