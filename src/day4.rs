extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::fmt;

fn main() {
    let input = "bgvyzdsv";
    let mut counter = 0u64;

    let mut sh = Md5::new();

    loop {
        counter += 1;
        let test_key = fmt::format(format_args!("{}{}", input, counter));

        sh.input_str(&test_key);
        let hash_result = sh.result_str();

        if hash_result.starts_with("00000") {
            println!("{}: {}", test_key, hash_result);
        }
        if hash_result.starts_with("000000") {
            println!("{}: {}", test_key, hash_result);
            break;
        }

        sh.reset();
     }
}
