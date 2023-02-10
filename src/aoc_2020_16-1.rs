/* Advent of Code 2020 Day 16 Puzzle 1 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_16.dat";

    let re_text = Regex::new(r"^\w+ *\w*: (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let re_blank = Regex::new(r"^$").unwrap();
    let re_values = Regex::new(r"^[\d|,]+$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut ranges: Vec<(u16, u16, u16, u16)> = vec![];
    let mut error_rate: u16 = 0;
    let mut my_ticket = true;
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if re_text.is_match(&line) {
            for t in re_text.captures_iter(&line.clone()) {
                println!("Ticket Text: {}-{} {}-{}", &t[1], &t[2], &t[3], &t[4]);
                ranges.push((
                    t[1].trim().parse::<u16>().expect("Wanted a number"),
                    t[2].trim().parse::<u16>().expect("Wanted a number"),
                    t[3].trim().parse::<u16>().expect("Wanted a number"),
                    t[4].trim().parse::<u16>().expect("Wanted a number"),
                ));
            }
        } else if re_blank.is_match(&line) {
            println!("Empty Line");
        } else if re_values.is_match(&line) && my_ticket {
            for v in line.split(',') {
                let _v = v.trim().parse::<u16>().expect("Wanted a number");
                //println!("v: {}", v);
            }
            //println!("My Ticket");
            my_ticket = false;
        } else if re_values.is_match(&line) && !my_ticket {
            for v in line.split(',') {
                let mut valid_ticket = false;
                let v = v.trim().parse::<u16>().expect("Wanted a number");
                for r in ranges.iter() {
                    if v >= r.0 && v <= r.1 || v >= r.2 && v <= r.3 {
                        valid_ticket = true;
                    }
                }
                if !valid_ticket {
                    error_rate += v;
                }
            }
        }
        println!("{}\t{}", index, &line);
    }
    println!("Error Rate {}", error_rate);
}
