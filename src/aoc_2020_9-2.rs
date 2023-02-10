/* Advent of Code 2020 Day 9 Puzzle 2 */

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
    let mut mat_no: u64 = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));
        let mut m = false;

        for code in re.captures_iter(&line) {
            let nn: u64 = code[1].trim().parse().expect("Wanted a number");
            if index >= 26 {
                xmas_1.drain(0..1); // shift out the top
                for i in xmas_1.iter() {
                    for j in xmas_1.iter() {
                        if i + j == nn {
                            //println!("Match!  i: {},\tj: {},\tnext number: {}", i, j, nn);
                            m = true;
                            break;
                        }
                    }
                }
            }
            xmas_1.push(nn);
            xmas_2.push(nn);
            //println!("{} {} {} {}", index, &code[0], &code[1], xmas_1.len());

            if !m && mat_no == 0 && index >= 26 {
                println!("No Match at next number: {}", nn);
                mat_no = nn;
                break;
            }
        }
    }
    println!("Running total to: {}", mat_no);
    loop {
        let mut rt: u64 = 0; // running total
        let mut sm: u64 = mat_no; // smallest
        let mut lg: u64 = 0; // largest
        let mut m = false;
        for i in xmas_2.iter() {
            if i < &sm {
                sm = *i;
            }
            if i > &lg {
                lg = *i;
            }
            rt += i;
            if rt == mat_no {
                println!("Running total matches, s: {} l: {} t:{}", sm, lg, sm + lg);
                m = true;
                break;
            } else if rt > mat_no {
                break;
            }
        }
        if m == true {
            break;
        }
        xmas_2.drain(0..1); // shift out the top and start over
    }
}
