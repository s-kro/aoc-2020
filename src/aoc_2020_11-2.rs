/* Advent of Code 2020 Day 11 Puzzle 2 */

use regex::Regex;
//use std::collections::HashMap;
//use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_11.dat";

    let re = Regex::new(r"^(L|\.){92}$").unwrap();
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

    loop { // two copies of the seating, we need one for reading and a separate one for writing
        let mut change_seats = 0; // ... becuase all the seating changes happen at once, so we
        let mut seats_taken = 0; //      alternate depending on wheather the loop counter is
        for (r, row) in seats[i % 2].clone().iter_mut().enumerate() {     // odd or even
            let rl = row.len() - 1;
            for (s, seat) in row.iter().enumerate() {
                let mut seat_mates = 0;

                for cc in -1i8..=1 {
                    for sc in -1i8..=1 {
                        let mut look_dist = 1;
                        loop {
                            if r as i8 + cc * look_dist < 0
                                || r as i8 + cc * look_dist > cl as i8
                                || s as i8 + sc * look_dist < 0
                                || s as i8 + sc * look_dist > rl as i8
                                || cc == 0 && sc == 0
                            {
                                //println!("Out of Range {}", look_dist);
                                break;
                            } else if seats[i % 2][(r as i8 + cc * look_dist) as usize]
                                [(s as i8 + sc * look_dist) as usize]
                                == '#'
                            {
                                seat_mates += 1;
                                break;
                            } else if seats[i % 2][(r as i8 + cc * look_dist) as usize]
                                [(s as i8 + sc * look_dist) as usize]
                                == 'L'
                            {
                                break;
                            }
                            look_dist += 1;
                        }
                    }
                }
                if seat_mates == 0 && seat == &'L' {
                    seats[(i + 1) % 2][r][s] = '#';
                    change_seats += 1;
                } else if seat_mates >= 5 && seat == &'#' {
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
