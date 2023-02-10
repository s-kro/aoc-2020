/* Advent of Code 2020 Day 6 Puzzle 2 */

use regex::Regex;
//use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_6.dat";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut group = String::new();
    let re = Regex::new(r"^\s*$").unwrap();
    let mut i: u32 = 0;

    for (_index, pax) in reader.lines().enumerate() {
        let pax = pax.unwrap(); // Ignore errors.
        if !re.is_match(&pax.as_str()) {
            group.push_str(" ");
            group.push_str(pax.as_str());
        } else {
            println!("{}", group);
            let mut m = 0;
            for l in 'a'..='z' {
                let mut all_pax = 0;
                let mut grp_sz = 0;
                for p in group.split_ascii_whitespace() {
                    grp_sz += 1;
                    for r in p.as_bytes() {
                        if *r as char == l {
                            all_pax += 1;
                            //println!("Matched: {}", &l);
                            break;
                        }
                    }
                }
                if all_pax == grp_sz {
                    println!("Matched: {}", &l);
                    m += 1;
                }
            }
            println!("Matches: {}", m);
            i += m;
            group.clear();
        }
        println!("Total Matches: {}", i);
    }
}
