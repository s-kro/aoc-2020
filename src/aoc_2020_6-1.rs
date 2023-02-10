/* Advent of Code 2020 Day 6 Puzzle 1 */

use regex::Regex;
//use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_6.dat";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut record = String::new();
    let re = Regex::new(r"^\s*$").unwrap();
    let mut i: u32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if !re.is_match(&line.as_str()) {
            record.push_str(" ");
            record.push_str(line.as_str());
        } else {
            println!("{}", record);
            let mut m = 0;
            for l in 'a'..='z' {
                println!("Testing letter: {}", l);
                for r in record.as_bytes() {
                    if *r as char == l {
                        m += 1;
                        println!("Matched: {}", &l);
                        break;
                    }
                }
            }
            println!("Matches: {}", m);
            i += m;
            record.clear();
        }
        println!("Total Matches: {}", i);
    }
}
