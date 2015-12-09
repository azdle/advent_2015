extern crate pcre;

use pcre::Pcre;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut input_file = File::open("input/day5.txt").unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();

    let mut good_list: Vec<&str> = Vec::new();
    let mut bad_list: Vec<&str>  = Vec::new();

    let mut r1 = Pcre::compile(r"(:?[aeiou].*){3}").unwrap();
    let mut r2 = Pcre::compile(r"(:?[a-zA-Z])\1").unwrap();
    let mut r3 = Pcre::compile(r"ab|cd|pq|xy").unwrap();

    for child in input.split("\n") {
        if r1.exec(child).is_none() {
            //println!("{} breaks rule 1.", child);
            bad_list.push(&child);
        } else if r2.exec(child).is_none() {
            //println!("{} breaks rule 2.", child);
            bad_list.push(&child);
        } else if !r3.exec(child).is_none() {
            //println!("{} breaks rule 3.", child);
            bad_list.push(&child);
        } else {
            //println!("{} is nice.", child);
            good_list.push(&child);
        }
    }

    println!("Nice Children: {}", good_list.len());
    println!("Naughty Children: {}", bad_list.len());

    let mut good_list: Vec<&str> = Vec::new();
    let mut bad_list: Vec<&str>  = Vec::new();

    let mut p2r1 = Pcre::compile(r"(..).*\1").unwrap();
    let mut p2r2 = Pcre::compile(r"(.).\1").unwrap();

    for child in input.split("\n") {
        if p2r1.exec(child).is_none() {
            //println!("{} breaks rule one. (pt2)", child);
            bad_list.push(&child);
        } else if p2r2.exec(child).is_none() {
            //println!("{} breaks rule one. (pt2)", child);
            bad_list.push(&child);
        } else {
            //println!("{} is nice. (pt2)", child);
            good_list.push(&child);
        }
    }

    println!("Nice Children: {} (pt2)", good_list.len());
    println!("Naughty Children: {} (pt2)", bad_list.len());
}
