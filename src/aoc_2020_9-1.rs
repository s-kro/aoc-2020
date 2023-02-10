/* Advent of Code 2020 Day 9 Puzzle 1 */

use regex::Regex;
//use std::collections::HashMap;
// use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_9.dat";

    let re = Regex::new(r"^(\d+)$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut xmas_1: Vec<u64> = Vec::new();
    let mut xmas_2: Vec<u64> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));
        let mut m = false;

        for code in re.captures_iter(&line) {
            let nn: u64 = code[1].trim().parse().expect("Wanted a number");
            if index >= 26 {
                xmas.drain(0..1); // shift out the top
                for i in xmas.iter() {
                    for j in xmas.iter() {
                        if i + j == nn {
                            //println!("Match!  i: {},\tj: {},\tnext number: {}", i, j, nn);
                            m = true;
                            break;
                        }
                    }
                }
            }
            xmas.push(nn);
            println!("{} {} {} {}", index, &code[0], &code[1], xmas.len());

            if !m {
                println!("No Match at next number: {}", nn);
                break;
            }
        }
    }
}
