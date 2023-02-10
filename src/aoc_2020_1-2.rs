/* Advent of Code 2020 Day 1 Puzzle 2 */

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "aoc_2020_1.dat";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut a: [i32; 200] = [0; 200];

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
        let input: i32 = line.trim().parse().expect("Wanted a number");

        a[index] = input;
        // println!("{}. {}", index + 1, line);
    }
    for r in a.iter() {
        for s in a.iter() {
            let x = 2020 - (s + r);
            //println!("{} {}", s, x);
            for t in a.iter() {
                if *t == x {
                    println!("\t{} {} {} {} {}", r, s, t, r + s + t, r * s * t);
                }
            }
        }
    }
}
