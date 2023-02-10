/* Advent of Code 2020 Day 1 Puzzle 1 */

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "dat/aoc_2020_1.dat";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut a: [u32; 200] = [0; 200];
    
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
	let input: u32 = line
        .trim()
        .parse()
        .expect("Wanted a number");

	a[index] = input;
       // println!("{}. {}", index + 1, line);
    }
    println!("number of elements in array: {}", a.len());
    for s in a.iter(){
	let x = 2020 - s;
	//println!("{} {}", s, x);
	for t in a.iter(){
	    if *t == x {
		println!("\t{} {} {}", s, t, s * t);
	    }
	}
    }
}

