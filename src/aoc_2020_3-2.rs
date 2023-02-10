/* Advent of Code 2020 Day 3 Puzzle 2 */

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_3.dat";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    /*
        Right 1, down 1.
        Right 3, down 1. (This is the slope you already checked.)
        Right 5, down 1.
        Right 7, down 1.
        Right 1, down 2.
    */

    let mut i: u32 = 0; // Not really needed
                        // [Right, Down, Hit tree count], each trip down the slope is called a "run"
    let mut runs = [[1, 1, 0], [3, 1, 0], [5, 1, 0], [7, 1, 0], [1, 2, 0]];

    for (index, line) in reader.lines().enumerate() {
        // "index" is how far down the hill we've taboggoned
        let line = line.unwrap(); // Ignore errors.

        for run in runs.iter_mut() {
            println!(
                "{}\t : {}. {} n{:?}\t{}",
                index / run[1],       // to 'skip' every second row (for the last run)
                line.chars().count(), // we'll simulte the repetition of the rows by taking
                ((index / run[1]) * run[0]) % line.chars().count(), // the remainder wrap to the beginning
                line.as_bytes()[((index / run[1]) * run[0]) % line.chars().count()],
                line
            );
            let tree = "#";
            if line.as_bytes()[((index / run[1]) * run[0]) % line.chars().count()]
                == tree.as_bytes()[0]
                && index % run[1] == 0
            // only count every second row for the last run
            {
                i += 1;
                run[2] += 1; // run[2]++  ?!?
                println!(
                    "i: {} run: {} step: {} count: {}",
                    i, &run[0], &run[1], &run[2]
                );
            }
        }
    }
    let mut p = 1; // product
    for run in runs.iter() {
        p *= run[2];
        println!("run:{}\tproduct: {}", &run[2], p);
    }
}
