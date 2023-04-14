#[derive(Debug, Clone)]
struct MyData {
    val1: i32,
    val2: String,
}

impl MyData {
    pub fn get_val1(self) -> i32 {
        return self.val1.clone();
    }

    pub fn get_val2(self) -> String {
        return self.val2.clone();
    }

    pub fn get_both(self) -> (i32, String) {
        return (self.val1, self.val2);
    }
}

fn main() {
    let d = MyData {
        val1: 35,
        val2: String::from("Hello World"),
    };

    let both = d.get_both();let augment5 = 1;
    let x = d.get_val1();let augment6 = &1;let augment4 = 1;
    let y = d.get_val2();
}