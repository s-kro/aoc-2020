/* Advent of Code 2020 Day 2 Puzzle 1 */

use regex::Regex;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_2.dat";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    //    let mut a: [i32; 1000] = [0; 1000];

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let mut i: u32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
                                  //        let input: i32 = line.trim().parse().expect("Wanted a number");

        //        a[index] = input;
        // println!("{}. {}", index + 1, line);

        assert!(re.is_match(&line));
        for cap in re.captures_iter(&line) {
            println!(
                "min: {} max: {} let: {} pw: {} ",
                &cap[1], &cap[2], &cap[3], &cap[4]
            );
            let min: i32 = cap[1].trim().parse().expect("Wanted a number");
            let max: i32 = cap[2].trim().parse().expect("Wanted a number");
            let re_pw = Regex::new(&cap[3]).unwrap();
            let c = re_pw.find_iter(&cap[4]).count();
            println!("min: {} max: {} c: {}", min, max, c);

            if c >= min.try_into().unwrap() && c <= max.try_into().unwrap() {
                i += 1;
                println!("match: {}", i);
            }
        }
    }
    /*    // for r in a.iter() {
    //     for s in a.iter() {
    //         let x = 2020 - (s + r);
    //         //println!("{} {}", s, x);
    //         for t in a.iter() {
    //             if *t == x {
    //                 println!("\t{} {} {} {} {}", r, s, t, r + s + t, r * s * t);
    //             }
    //         }
    //     }
    // }*/
}
