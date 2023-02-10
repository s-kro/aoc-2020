/* Advent of Code 2020 Day 10 Puzzle 2 */

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
    jolts.push(jolts[jolts.len() - 1] + 3); // for the device
    let mut i_prev: u32 = 0;
    let ts = [0, 1, 1, 2, 4, 7, 13, 24, 44, 81, 148, 274]; // tribonacci sequence
    let mut ti = 1; // index into tribonacci sequence
    let mut combinations: u64 = 1;
    for i in jolts.iter() {
        println!("{}", i);
        if i - i_prev == 1 {
            ti += 1;
        }
        if i - i_prev == 3 { // a jump of 3 jolts terminates the  
            combinations *= ts[ti]; // tribonacci 'branching' so do the
            println!(               // calculation and reset
                "ti: {}\ttn: {} total comb: {}",
                ti, ts[ti], combinations
            );
            ti = 1;
        }
        i_prev = *i;
    }
}
