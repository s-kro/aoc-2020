/* Advent of Code 2020 Day 8 Puzzle 1 */

use regex::Regex;
//use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_8.dat";

    let re = Regex::new(r"^(\w{3}) ((?:\+|\-)\d+)$").unwrap();
    let mut last_line = 0;
    let mut vec: Vec<(usize, String, i16, bool)> = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));
        for code in re.captures_iter(&line) {
            let i: i16 = code[2].trim().parse().expect("Wanted a number");
            //println!("{} {} {}", index, &code[1], &code[2]);
            vec.push((index, code[1].to_string(), i, false));
        }
        last_line = index;
    }

    let mut ip: usize = 0;
    let mut acc: i16 = 0;
    loop {
        println!("{} {} {} {}", vec[ip].0, vec[ip].1, vec[ip].2, vec[ip].3);
        if ip == last_line {
            break;
        }
        if !vec[ip].3 {
            vec[ip].3 = true;
            if vec[ip].1 == "acc" {
                acc += vec[ip].2; //.try_into().unwrap();
            }
            if vec[ip].1 == "jmp" {
                ip = (ip as i16 + vec[ip].2).try_into().unwrap();
                println!("ip {}", ip);
            } else {
                ip += 1;
            }
        } else {
            break;
        }
    }
    println!("accumulator: {}", acc);
}
