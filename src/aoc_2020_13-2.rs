/* Advent of Code 2020 Day 13 Puzzle 2 */

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn aoc_2020_13() {
    let filename = "src/aoc_2020_13.dat";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut table = "".to_string();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        match index {
            0 => {} // not required
            1 => table = line,
            _ => println!("File read error"),
        }
    }
    //    println!("{}", table);
    let mut inc: i64 = 1;
    let mut cr: i64 = 0;

    // from https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Computation
    // "Search by sieving" is fast enough for this problem

    for t in table.split(',').enumerate() {
        if t.1 != "x" {
            let modu = t.1.parse::<i64>().unwrap();
            if inc == 1 {
                cr = t.0 as i64; // Chinese remainder?
                inc *= modu;
            } else {
                let mut i = 0;

                loop {
                    if t.0 as i64 % modu == (cr + i * inc) % modu {
                        cr += i * inc;
                        inc *= modu;

                        println!(
                            "a {}, n {},\trem {},\tinc {},\tans {}",
                            t.0,
                            t.1,
                            cr,
                            inc,
                            inc - cr
                        );

                        break;
                    }
                    i += 1;
                }
            }
        }
    }
}
