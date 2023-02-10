/* Advent of Code 2020 Day 12 Puzzle 2 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn aoc_2020_12_2() {
    let filename = "src/aoc_2020_12.dat";

    let re = Regex::new(r"^(N|S|E|W|F|R|L)(\d+)$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut heading: i32 = 90;
    let mut pos_n: i32 = 0;
    let mut pos_s: i32 = 0;
    let mut pos_e: i32 = 0;
    let mut pos_w: i32 = 0;
    let mut wp_ns: i32 = 1; // North pos
    let mut wp_ew: i32 = -10; // West pos

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));
        println!("{}\t{}", index, &line);
        for instruction in re.captures(&line) {
            match &instruction[1] "L" => {
                heading -= &instruction[2].parse().unwrap();
                heading %= 360;
                if heading < 0 {
                    heading += 360;
                }
                //   println!("{}, {}", &instruction[1], heading);
            } else if &instruction[1] == "R" {
                heading += &instruction[2].parse().unwrap();
                heading %= 360;
                //  println!("{}, {}", &instruction[1], heading);
            }  &instruction[1] == "F" {
                match heading
		    0 => {
                    pos_n += &instruction[2].parse().unwrap();
                } 90 => {
                    pos_e += &instruction[2].parse().unwrap();
                } 180 => {
                    pos_s += &instruction[2].parse().unwrap();
                } 270 =>{
                    pos_w += &instruction[2].parse().unwrap();
                }
            } else if &instruction[1] == "N" {
                pos_n += &instruction[2].parse().unwrap();
            } else if &instruction[1] == "E" {
                pos_e += &instruction[2].parse().unwrap();
            } else if &instruction[1] == "S" {
                pos_s += &instruction[2].parse().unwrap();
            } else if &instruction[1] == "W" {
                pos_w += &instruction[2].parse().unwrap();
            }
            //   println!(
            //      "{}, {} {} {} {}",
            //      &instruction[1], pos_n, pos_e, pos_s, pos_w
            // );
        }
        println!("M.D. {}", (pos_n - pos_s).abs() + (pos_e - pos_w).abs());
    }
}
