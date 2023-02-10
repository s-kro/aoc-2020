/* Advent of Code 2020 Day 18 Puzzle 2 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_18.dat";

    let re = Regex::new(r"^(\d|\(|\)| |\*|\+)+$").unwrap();
    let re_bracket = Regex::new(r"(?:(.*)\(([\d+| |\+|\*]+)\)(.*))").unwrap();
    let re_addition = Regex::new(r"(?:(.* |)((\d+) \+ (\d+))( .*|))").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut total_sum: i64 = 0;

    for (index, line) in reader.lines().enumerate() {
        let mut line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));
        println!("{}\t{}", index, &line);

        loop {
            if re_bracket.is_match(&line) {
                for b in re_bracket.captures_iter(&line.clone()) {
                    println!("bracket cap: {}[{}]{}", &b[1], &b[2], &b[3]);
                    let mut t: i64 = 0;
                    let mut op = "";

                    let mut br = b[2].to_string(); // inside of an innermost bracket
                    loop {
                        if re_addition.is_match(&br) {
                            for a in re_addition.captures_iter(&br.clone()) {
                                println!(
                                    "addition cap: {}({}){}, {} {}",
                                    &a[1], &a[2], &a[5], &a[3], &a[4]
                                );
                                br = format!(
                                    "{}{}{}",
                                    &a[1],
                                    a[3].trim().parse::<i64>().expect("Wanted a number")
                                        + a[4].trim().parse::<i64>().expect("Wanted a number"),
                                    &a[5]
                                );
                            }
                        } else {
                            break;
                        }
                    }
                    for (i, d) in br.split_ascii_whitespace().enumerate() {
                        if i == 0 {
                            t = d.trim().parse().expect("Wanted a number");
                        } else if i % 2 == 1 {
                            op = d;
                        } else {
                            if op == "+" {
                                t += d.trim().parse::<i64>().expect("Wanted a number");
                                println!("add t: {} ** Should not be here", t);
                            } else if op == "*" {
                                t *= d.trim().parse::<i64>().expect("Wanted a number");
                                println!("mul t: {}", t);
                            }
                        }
                    }
                    line = format!("{}{}{}", &b[1], t, &b[3]);
                }
            } else {
                println!("Final line: {}", &line);
                let mut t: i64 = 0;
                let mut op = "";

                loop {
                    if re_addition.is_match(&line) {
                        for a in re_addition.captures_iter(&line.clone()) {
                            line = format!(
                                "{}{}{}",
                                &a[1],
                                a[3].parse::<i64>().expect("Wanted a number")
                                    + a[4].parse::<i64>().expect("Wanted a number"),
                                &a[5]
                            );
                        }
                    } else {
                        break;
                    }
                }

                for (i, d) in line.split_ascii_whitespace().enumerate() {
                    if i == 0 {
                        t = d.trim().parse().expect("Wanted a number");
                    } else if i % 2 == 1 {
                        op = d;
                    } else {
                        if op == "+" {
                            t += d.trim().parse::<i64>().expect("Wanted a number");
                            println!("add t: {} ** Sould Not be Here", t);
                        } else if op == "*" {
                            t *= d.trim().parse::<i64>().expect("Wanted a number");
                            println!("mul t: {}", t);
                        }
                    }
                }
                total_sum += t;
                println!("\tTotal: {}", total_sum);
                break;
            }
        }
    }
}
