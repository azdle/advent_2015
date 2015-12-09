extern crate regex;
use regex::Regex;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut input_file = File::open("input/day6.txt").unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();

    let mut lights = vec![vec![false; 1000]; 1000];
    let mut lights2 = vec![vec![0u64; 1000]; 1000];

    let re = Regex::new(r"([a-zA-Z ]+) ([0-9,]+),([0-9,]+) through ([0-9,]+),([0-9,]+)").unwrap();
    for cap in re.captures_iter(&input) {
        let command = cap.at(1).unwrap();
        let x1 = cap.at(2).unwrap().parse::<usize>().unwrap();
        let y1 = cap.at(3).unwrap().parse::<usize>().unwrap();
        let x2 = cap.at(4).unwrap().parse::<usize>().unwrap();
        let y2 = cap.at(5).unwrap().parse::<usize>().unwrap();

        //println!("{}: {},{}-{},{}", command, x1, y1, x2, y2);

        for x in x1..x2+1 {
            for y in y1..y2+1 {
                match command {
                    "turn on" => {
                        lights[x][y] = true;
                        lights2[x][y] += 1;
                    },
                    "turn off" => {
                        lights[x][y] = false;

                        if lights2[x][y] > 0 {
                            lights2[x][y] -= 1;
                        }
                    },
                    "toggle" => {
                        lights[x][y] = !lights[x][y];
                        lights2[x][y] += 2;
                    },
                    _ => panic!("Invalid Command")
                }
            }
        }
    }

    let mut count = 0u32;

    for col in lights {
        for cell in col {
            match cell {
                true => count += 1,
                false => (),
            }
        }
    }

    println!("There are {} lights on.", count);


    let mut net_brightness = 0u64;

    for col in lights2 {
        for cell in col {
            net_brightness += cell;
        }
    }

    println!("The total brightness is {}.", net_brightness);
}
