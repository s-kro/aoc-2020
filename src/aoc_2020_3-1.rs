/*Advent of Code 2020 Day 3 Puzzle 1*/

use regex::Regex;
//use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::from_utf8;

fn main() {
    let filename = "src/aoc_2020_3.dat";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    //    let mut a: [i32; 1000] = [0; 1000];

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    //let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let mut i: u32 = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
                                  //        let input: i32 = line.trim().parse().expect("Wanted a number");

        //        a[index] = input;
        println!(
            "{}. {:?} {}",
            (((index + 1) * 3 - 2) - 1) % line.chars().count() + 1,
            line.as_bytes()[(((index + 1) * 3 - 2) - 1) % line.chars().count()],
            line
        );
        let tree = "#";
        if line.as_bytes()[(((index + 1) * 3 - 2) - 1) % line.chars().count()] == tree.as_bytes()[0]
        {
            i += 1;
            println!("i: {}", i);
        }
        //

        //assert!(re.is_match(&line));

        //      if c != d {
        //          i += 1;
        //          println!("i: {}", i);
        //      }
    }
}
