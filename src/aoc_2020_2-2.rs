/* Advent of Code 2020 Day 2 Puzzle 2 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_2.dat";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let mut i: u32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.

        assert!(re.is_match(&line));
        for cap in re.captures_iter(&line) {
            println!(
                "fst: {} max: {} let: {} pw: {} ",
                &cap[1], &cap[2], &cap[3], &cap[4]
            );
            let fst: i32 = cap[1].trim().parse().expect("Wanted a number");
            let sec: i32 = cap[2].trim().parse().expect("Wanted a number");

            let reg = format!("^\\w{{{}}}{}", fst - 1, &cap[3]);

            let re_pw1 = Regex::new(format!("^\\w{{{}}}{}", fst - 1, &cap[3]).as_str()).unwrap();
            let re_pw2 = Regex::new(format!("^\\w{{{}}}{}", sec - 1, &cap[3]).as_str()).unwrap();
            //let re_pw1 = Regex::new(&reg.as_str()).unwrap();
            let c = re_pw1.is_match(&cap[4]);
            let d = re_pw2.is_match(&cap[4]);
            println!("fst: {} max: {} c: {} reg: {}", fst, sec, c, &reg.as_str());

            if c != d {
                i += 1;
                println!("i: {}", i);
            }
        }
    }
}
