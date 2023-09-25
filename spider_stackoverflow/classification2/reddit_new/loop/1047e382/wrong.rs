extern crate walkdir;
use walkdir::WalkDir;

fn main() {
    let user = "username";
    let homedir = WalkDir::new("/home/".to_string() + user + "/");

    for entry in homedir {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }

    println!("Files: {}", homedir.into_iter().count());
}