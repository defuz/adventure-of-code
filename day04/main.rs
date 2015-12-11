extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

const PREFIX : &'static str = "iwrupvqb";

fn find(digest_prefix: &str) -> u32 {
    for i in 0.. {
        let mut digest = Md5::new();
        digest.input_str(&format!("{}{}", PREFIX, i));
        if &digest.result_str()[..digest_prefix.len()] == digest_prefix {
            return i
        }
    }
    unreachable!()
}

fn main() {
    println!("Part one: {}", find("00000"));
    println!("Part two: {}", find("000000"));
}
