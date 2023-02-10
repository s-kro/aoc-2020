/* Advent of Code 2020 Day 12 Puzzle 2 */

use core::mem::swap;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn aoc_2020_12() {
    let filename = "src/aoc_2020_12.dat";

    let re = Regex::new(r"^(N|S|E|W|F|R|L)(\d+)$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    //let mut heading: i32 = 90;
    let mut pos_ns: i32 = 0;
    let mut pos_ew: i32 = 0;
    let mut wp_ns: i32 = 1; // North pos
    let mut wp_ew: i32 = -10; // West pos

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));
        println!("{}\t{}", index, &line);
        if let Some(instruction) = re.captures(&line) {
            match &instruction[1] {
                "L" => match &instruction[2] {
                    "90" => {
                        swap(&mut wp_ns, &mut wp_ew);
                        wp_ns = -wp_ns;
                    }
                    "180" => {
                        wp_ns = -wp_ns;
                        wp_ew = -wp_ew;
                    }
                    "270" => {
                        swap(&mut wp_ns, &mut wp_ew);
                        wp_ew = -wp_ew;
                    }
                    &_ => println!("L ERROR"),
                },

                "R" => match &instruction[2] {
                    "90" => {
                        swap(&mut wp_ns, &mut wp_ew);
                        wp_ew = -wp_ew;
                    }
                    "180" => {
                        wp_ns = -wp_ns;
                        wp_ew = -wp_ew;
                    }
                    "270" => {
                        swap(&mut wp_ns, &mut wp_ew);
                        wp_ns = -wp_ns;
                    }
                    &_ => println!("R ERROR"),
                },

                "F" => {
                    pos_ns += wp_ns * &instruction[2].parse().unwrap();
                    pos_ew += wp_ew * &instruction[2].parse().unwrap();
                }
                "N" => wp_ns += &instruction[2].parse().unwrap(),
                "E" => wp_ew -= &instruction[2].parse().unwrap(),
                "S" => wp_ns -= &instruction[2].parse().unwrap(),
                "W" => wp_ew += &instruction[2].parse().unwrap(),
                _ => println!("ERROR instruction {}", &instruction[1]),
            }
        }
        println!("M.D. {}", pos_ns.abs() + pos_ew.abs());
    }
}
