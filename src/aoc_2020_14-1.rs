/* Advent of Code 2020 Day 14 Puzzle 1 */

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn aoc_2020_14() {
    let filename = "src/aoc_2020_14.dat";

    let re_bm = Regex::new(r"^mask = ([X|0|1]{36})$").unwrap();
    let re_instr = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    let mut mem_sum: u64 = 0;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut mem = HashMap::new();
    let mut mask: String = "".to_string();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if re_bm.is_match(&line) {
            //println!("{}", &line);
            for m in re_bm.captures_iter(&line) {
                mask = m[1].to_string().chars().rev().collect(); // need the MSB at the other end
                println!("{}", mask); //                             of the mask string
            }
        } else if re_instr.is_match(&line) {
            //println!("{}", &line);
            for i in re_instr.captures_iter(&line) {
                println!("{}", &i[2]);
                let mut overwrite: u64 = 0;
                for bit in mask.chars().enumerate() {
                    //    println!("{} {}", &bit.0, &bit.1);
                    if bit.1 == '1' {
                        overwrite += 2_u64.pow(bit.0 as u32);
                    } else if bit.1 == 'X' {
                        let b = (i[2].trim().parse::<u64>().expect("Wanted a Number") >> bit.0) % 2;
                        overwrite += b * 2_u64.pow(bit.0 as u32);
                    }
                }
                mem.insert(i[1].to_string(), overwrite);
            }
        }
    }
    for (add, value) in mem {
        println!("Address: {}, Value {}", add, value);
        mem_sum += value;
    }
    println!("Total Mem value: {}", mem_sum);
}
