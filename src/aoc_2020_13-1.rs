/* Advent of Code 2020 Day 13 Puzzle 1 */

use std::io::{BufRead, BufReader};
use std::{cmp::min, fs::File};

pub fn aoc_2020_13() {
    let filename = "src/aoc_2020_13.dat";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut time = 0;
    let mut table = "".to_string();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        match index {
            0 => time = line.parse().unwrap(),
            1 => table = line,
            _ => println!("File read error"),
        }
    }

    // println!("{}\t{}", time, table);
    let mut low_wt = 646; // an arbitrarily high wait time
    let mut low_prod = 0;
    for t in table.split(',') {
        if t == "x" {
        } else {
            let r = time % t.parse::<i32>().unwrap(); // how much we missed the last bus by
            let wt = t.parse::<i32>().unwrap() - r; // wait time to the next bus on that sched
            low_wt = min(low_wt, wt);
            if low_wt == wt {
                low_prod = t.parse::<i32>().unwrap() * wt;
            }
            println!(
                "t: {}, r: {}, low_wt: {}, low_prod: {}",
                t, r, low_wt, low_prod
            );
        }
    }
}
