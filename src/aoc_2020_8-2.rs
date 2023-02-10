/* Advent of Code 2020 Day 8 Puzzle 2 */

use regex::Regex;
//use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_8.dat";

    let re = Regex::new(r"^(\w{3}) ((?:\+|\-)\d+)$").unwrap();
    let mut last_line = 0;
    let mut vec: Vec<(usize, String, i16, i16, bool)> = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));
        for code in re.captures_iter(&line) {
            let i: i16 = code[2].trim().parse().expect("Wanted a number");
            //println!("{} {} {}", index, &code[1], &code[2]);
            vec.push((index, code[1].to_string(), i, 1, false));
        }
        last_line = index;
    }
    println!("last line {}", last_line);

    let mut ip: usize = 0; // instruction pointer
    let mut ip_prev: usize; // prev instruction pointer
                            //    let mut rp: i16 = 1; // reverse pointer
    let mut acc: i16 = 0;
    //let mut dir: i16 = 1; // direction (-1 = reverse)
    let mut ch_dir_ct = 0;
    let mut last_instr = 0;
    let mut nop_jmp_srch = true;
    let mut skip = false;

    loop {
        if ip >= last_line {
            println!("exit bottom {}", acc);
            break;
        }

        println!(
            "{} {} {} {} {}",
            vec[ip].0, vec[ip].1, vec[ip].2, vec[ip].3, vec[ip].4
        );

        if !vec[ip].4 || !skip || nop_jmp_srch {
            //println!("executing: {}", ip);
            vec[ip].4 = true;
            if vec[ip].1 == "acc" {
                acc += vec[ip].2;
                println!("acc: {}", acc);
            }
            ip_prev = ip;
            println!("last_instr {}", last_instr);

            if vec[ip].1 == "jmp" {
                if nop_jmp_srch == false {
                    ip = (ip as i16 + vec[ip].2).try_into().unwrap();
                } else {
                    nop_jmp_srch = false;
                    last_instr = ip;
                    ip += 1;
                }
            } else if vec[ip].1 == "nop" {
                if nop_jmp_srch == false {
                    ip += 1;
                } else {
                    nop_jmp_srch = false;
                    last_instr = ip;
                    ip = (ip as i16 + vec[ip].2).try_into().unwrap();
                }
            } else {
                ip += 1;
            }
            if ip_prev == last_instr && !skip {
                nop_jmp_srch = true;
                skip = true; // prevent last instr from just tracking loop
                println!("skip true at ip {}", ip);
            }
        } else {
            println!("\t\tchg dir accumulator: {}", acc);

            ip = 0;
            acc = 0;
            skip = false;
            nop_jmp_srch = false;

            ch_dir_ct += 1;
            if ch_dir_ct > 64 {
                break;
            }
        }
    }
    println!("exit accumulator: {}, {} dead ends", acc, ch_dir_ct);
}
