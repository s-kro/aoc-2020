/* Advent of Code 2020 Day 5 Puzzle 2 */

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
    let mut a: [u32; 1024] = [0; 1024];

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let mut i: u32 = 10;
        let mut seat: u32 = 0;
        for b in line.as_bytes() {
            i -= 1;
            if b == &"B".as_bytes()[0] || b == &"R".as_bytes()[0] {
                seat = seat + 2_u32.pow(i);
            }
        }
        a[index] = seat;
        if seat > high_seat {
            high_seat = seat;
        } else if seat < low_seat {
            low_seat = seat;
        }
    }
    for seat in low_seat..high_seat {
        let mut b_taken = 0;
        for taken in a.iter() {
            if &seat == taken {
                //println!("seat {} taken", seat);
                b_taken = 1;
                break;
            }
        }
        if b_taken == 0 {
            println!("seat {} is empty!", seat);
        }
    }
    println!("High seat ID: {}", high_seat);
    println!("Low seat ID: {}", low_seat);
}
