/* Advent of Code 2020 Day 11 Puzzle 1 */

use regex::Regex;
//use std::collections::HashMap;
// use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_11.dat";

    let re = Regex::new(r"^(L|\.){92}$").unwrap();
    //let re = Regex::new(r"^(L|\.){10}$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut seats: [Vec<Vec<char>>; 2] = [vec![], vec![]];

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));
        seats[0].push(line.chars().collect());
        seats[1].push(line.chars().collect());
        println!("{}\t{}", index, &line);
    }
    let cl = seats[0].len() - 1;
    let mut i = 0;

    loop {
        let mut change_seats = 0;
        let mut seats_taken = 0;
        for (r, row) in seats[i % 2].clone().iter_mut().enumerate() {
            let rl = row.len() - 1;
            //println!("cl: {} rl: {}", cl, rl);

            let c_start = if r == 0 { 0 } else { r - 1 };
            let c_end = if r == cl { cl } else { r + 1 };

            for (s, seat) in row.iter().enumerate() {
                let s_start = if s == 0 { 0 } else { s - 1 };
                let s_end = if s == rl { rl } else { s + 1 };

                let mut seat_mates = 0;
                let mut _seats_checked = 0;

                for cc in c_start..=c_end {
                    for sc in s_start..=s_end {
                        if !(cc == r && sc == s) {
                            _seats_checked += 1;
                            if seats[i % 2][cc][sc] == '#' {
                                seat_mates += 1;
                            }
                        }
                    }
                }
                if seat_mates == 0 && seat == &'L' {
                    seats[(i + 1) % 2][r][s] = '#';
                    change_seats += 1;
                } else if seat_mates >= 4 && seat == &'#' {
                    seats[(i + 1) % 2][r][s] = 'L';
                    change_seats += 1;
                } else {
                    seats[(i + 1) % 2][r][s] = *seat;
                    if seat == &'#' {
                        seats_taken += 1;
                    }
                }
            }
            // println!("{} {:?}, i: {}", r, &row, i);
        }
        i += 1;
        println!(
            "Change seats: {}, Seats taken {}",
            change_seats, seats_taken
        );
        if change_seats == 0 {
            break;
        }
    }
}
