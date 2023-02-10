/* Advent of Code 2020 Day 14 Puzzle 2 */

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
            }
        } else if re_instr.is_match(&line) {
            //println!("{}", &line);
            for i in re_instr.captures_iter(&line) {
                let mut add_list: [Vec<u64>; 2] = [vec![0], vec![]];
                let m = i[1].trim().parse::<u64>().expect("Wanted a Number");
                for bit in mask.chars().enumerate() {
                    let bit_value = 2_u64.pow(bit.0 as u32);
                    for add in add_list[(bit.0 % 2)].clone() {
                        match bit.1 {
                            '0' => {
                                let b = (m >> bit.0) % 2;
                                add_list[(bit.0 + 1) % 2].push(add + b * bit_value);
                            }
                            '1' => add_list[(bit.0 + 1) % 2].push(add + bit_value),
                            'X' => {
                                add_list[(bit.0 + 1) % 2].push(add);
                                add_list[(bit.0 + 1) % 2].push(add + bit_value);
                            }
                            _ => println!("Error"),
                        }
                    }
                    add_list[bit.0 % 2] = vec![];
                }

                for add in add_list[36 % 2].clone() {
                    mem.insert(add, i[2].trim().parse::<u64>().expect("Wanted a Number"));
                }
            }
        }
    }
    for (add, value) in mem {
        println!("Address: {}, Value {}", add, value);
        mem_sum += value;
    }
    println!("Total Mem value: {}", mem_sum);
}
