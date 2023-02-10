/* Advent of Code 2020 Day 10 Puzzle 1 */

use regex::Regex;
//use std::collections::HashMap;
// use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_10.dat";

    let re = Regex::new(r"^(\d+)$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut jolts: Vec<u32> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));

        for code in re.captures_iter(&line) {
            jolts.push(code[1].trim().parse().expect("Wanted a number"));
        }
    }
    jolts.sort();
    let mut i_prev: u32 = 0;
    let mut d = [0, 0, 0, 1];
    for i in jolts.iter() {
        println!("{}", i);
        d[(i - i_prev) as usize] += 1;
        i_prev = *i;
    }
    println!("{} {} {} {} {}", d[0], d[1], d[2], d[3], d[1] * d[3]);
}
