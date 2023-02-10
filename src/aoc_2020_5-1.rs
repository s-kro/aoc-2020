/* Advent of Code 2020 Day 5 Puzzle 1 */

//use regex::Regex;
//use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_5.dat";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut high_seat = 0;
    let mut low_seat = 1024;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  //  let line = "FFFBBBFRRR";
        let mut i: u32 = 10;
        let mut seat: u32 = 0;
        println!("index, line {}", /*index,*/ line);
        for b in line.as_bytes() {
            i -= 1;
            if b == &"B".as_bytes()[0] || b == &"R".as_bytes()[0] {
                seat = seat + 2_u32.pow(i);
            }
            //println!("char {}, seat {}", b, seat);
        }
        if seat > high_seat {
            high_seat = seat;
        } else if seat < low_seat {
            low_seat = seat;
        }
    }
    println!("High seat ID: {}", high_seat);
    println!("Low seat ID: {}", low_seat);
}
