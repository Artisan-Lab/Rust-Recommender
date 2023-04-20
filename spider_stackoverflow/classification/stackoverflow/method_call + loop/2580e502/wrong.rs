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
        for package in self.packages.iter_mut() {
            if package.id == 1 {
                self.send(package);
            }
            println!("{}", package.id);
        }
    }
    fn send(&self, package: &mut Package) {
        package.id = 23;
    }
}

fn main() {
    let mut manager = Manager::new();
    manager.run();
}
// iter()循环使用mut对象 不得不循环里使用可变引用同时还需要更多的可变引用