/* Advent of Code 2020 Day 18 Puzzle 1 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_18.dat";

    let re = Regex::new(r"^(\d|\(|\)| |\*|\+)+$").unwrap();
    let re_bracket = Regex::new(r"(?:(.*)\(([\d+| |\+|\*]+)\)(.*))+").unwrap();
    let re_addition = Regex::new(r"(?:(.*)([\d+ \+ \d+)(.*))+").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut gs: i64 = 0;

    for (index, line) in reader.lines().enumerate() {
        let mut line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));
        println!("{}\t{}", index, &line);

        loop {
            let mut c = line.clone();
            if re_bracket.is_match(&line) {
                let br = re_bracket.captures_iter(&line);
                for b in br {
                    println!(" {}{}{}", &b[1], &b[2], &b[3]);
                    let mut t: i64 = 0;
                    let mut op = "";
                    for (i, d) in b[2].split_ascii_whitespace().enumerate() {
                        if i == 0 {
                            t = d.trim().parse().expect("Wanted a number");
                        } else if i % 2 == 1 {
                            op = d;
                        } else {
                            if op == "+" {
                                t += d.trim().parse::<i64>().expect("Wanted a number");
                                //println!("add t: {}", t);
                            } else if op == "*" {
                                t *= d.trim().parse::<i64>().expect("Wanted a number");
                                //println!("mul t: {}", t);
                            }
                        }
                    }
                    c = format!("{}{}{}", &b[1], t, &b[3]);
                }
                line = c;
            } else {
                println!("Final line: {}", &line);
                let mut t: i64 = 0;
                let mut op = "";

                for (i, d) in line.split_ascii_whitespace().enumerate() {
                    if i == 0 {
                        t = d.trim().parse().expect("Wanted a number");
                    } else if i % 2 == 1 {
                        op = d;
                    } else {
                        if op == "+" {
                            t += d.trim().parse::<i64>().expect("Wanted a number");
                            //println!("add t: {}", t);
                        } else if op == "*" {
                            t *= d.trim().parse::<i64>().expect("Wanted a number");
                            //println!("mul t: {}", t);
                        }
                    }
                }
                gs += t;
                println!("\tgs: {}", gs);
                break;
            }
        }
    }
}
