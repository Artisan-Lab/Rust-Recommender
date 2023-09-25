use md5::{Md5, Digest};
use sha1::{Sha1};
use text_io::read;

fn main() {
    println!("[1] MD5 [2] SHA-1");
    let hash_type: i32 = read!();
    println!("Text to hash: ");
    let input_text: String = read!();

    if hash_type == 1{
        let mut hasher_md5 = Md5::new();
        hasher_md5.update(input_text);
        let result = hasher_md5.finalize();
        println!("Result: {:x}", result);
    }
    if hash_type == 2{
        let mut hasher_sha1 = Sha1::new();
        hasher_sha1.update(input_text);
        let result = hasher_sha1.finalize();
        println!("Result: {:x}", result);
    }
}