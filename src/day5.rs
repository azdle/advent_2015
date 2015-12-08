extern crate regex;
use regex::Regex;
use std::io::prelude::*;
use std::fs::File;

use std::process::Command;

fn has_repeats(name: &str) -> bool {
    let mut hist = 0u8;

    for byte in name.as_bytes() {
        if hist == *byte {
            return true;
        }

        hist = *byte;
    }

    false
}


fn main() {
    let mut input_file = File::open("input/day5.txt").unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();

    let mut good_list: Vec<&str> = Vec::new();
    let mut bad_list: Vec<&str>  = Vec::new();

    let r1 = Regex::new(r"(:?[aeiou].*){3}").unwrap();
    //let r2 = Regex::new(r"(:?[a-zA-Z])\1").unwrap();
    let r3 = Regex::new(r"ab|cd|pq|xy").unwrap();

    for child in input.split("\n") {


        if !r1.is_match(child) {
            println!("{} breaks rule 1.", child);
            bad_list.push(&child);
        } else if !has_repeats(child) {
            println!("{} breaks rule 2.", child);
            bad_list.push(&child);
        } else if r3.is_match(child) {
            println!("{} breaks rule 3.", child);
            bad_list.push(&child);
        } else {
            println!("{} is good.", child);
            good_list.push(&child);
        }

    }

    println!("Good Children: {}", good_list.len());
    println!("Bad Children: {}", bad_list.len());

    // pt2 -- lazyness
    // rust doesn't have backreferences in its regex.
    let output = Command::new("sh").arg("-c")
                       .arg("grep \"\\(..\\).*\\1\" input/day5.txt | grep \"\\(.\\).\\1\"")
                       .output().unwrap();;
    let output_str = String::from_utf8_lossy(&output.stdout);
    let count = output_str.lines().count();

    println!("Pt2 Count: {}", count);
}
