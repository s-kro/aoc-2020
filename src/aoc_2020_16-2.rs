/* Advent of Code 2020 Day 16 Puzzle 2 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_16.dat";

    let re_text = Regex::new(r"^(\w+ *\w*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let re_dptr = Regex::new(r"^departure \w+").unwrap();
    let re_values = Regex::new(r"^[\d|,]+$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut ranges: Vec<(String, u16, u16, u16, u16, Vec<usize>, u16)> = vec![];
    let mut valid_tk: Vec<String> = vec![];
    let mut error_rate: u16 = 0;
    let mut my_ticket_sem = true;
    let mut my_ticket = String::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if re_text.is_match(&line) {
            for t in re_text.captures_iter(&line.clone()) {
                println!("{}: {}-{} {}-{}", &t[1], &t[2], &t[3], &t[4], &t[5]);

                ranges.push((
                    t[1].to_string(),
                    t[2].trim().parse::<u16>().expect("Wanted a number"),
                    t[3].trim().parse::<u16>().expect("Wanted a number"),
                    t[4].trim().parse::<u16>().expect("Wanted a number"),
                    t[5].trim().parse::<u16>().expect("Wanted a number"),
                    vec![],
                    20,
                ));
            }
        } else if line.is_empty() {
        } else if re_values.is_match(&line) && my_ticket_sem {
            my_ticket = line.clone();
            my_ticket_sem = false;
        } else if re_values.is_match(&line) && !my_ticket_sem {
            let mut valid_ticket = true;
            line.split(',').for_each(|v| {
                let mut valid_num = false;
                let v = v.trim().parse::<u16>().expect("Wanted a number");
                for r in ranges.iter() {
                    if (r.1..=r.2).contains(&v) || (r.3..=r.4).contains(&v) {
                        valid_num = true;
                    }
                }
                if !valid_num {
                    error_rate += v;
                    valid_ticket = false;
                }
            });
            if valid_ticket {
                valid_tk.push(line.clone());
            }
        }
        // println!("{}\t{}", index, &line);
    }
    println!("\nError Rate {}\n", error_rate);

    let mut need_another_loop = true;
    while need_another_loop {
        need_another_loop = false;
        let invalid = ranges.to_owned();
        ranges.iter_mut().for_each(|r| {
            for tk in valid_tk.iter() {
                tk.split(',').enumerate().for_each(|(i, v)| {
                    let v = v.trim().parse::<u16>().expect("Wanted a number");
                    let mut inval = false;
                    if (r.1..=r.2).contains(&v) || (r.3..=r.4).contains(&v) {
                        invalid.iter().for_each(|s| {
                            if usize::from(s.6) == i && r.0 != s.0 {
                                inval = true;
                            }
                        });
                    } else {
                        inval = true;
                    }
                    if inval {
                        r.5.push(i);
                        r.5.sort_unstable();
                        r.5.dedup();
                    }
                });
            }

            if r.5.len() == 19 {
                for q in 0..=19 {
                    let mut q_is_in_s = false;
                    r.5.iter().for_each(|s| {
                        if q == *s {
                            q_is_in_s = true;
                        }
                        // println!("{}", s);
                    });
                    if !q_is_in_s {
                        println!("{} {} not invalid", r.0, q);
                        r.6 = q as u16;
                    }
                }
            } else {
                need_another_loop = true;
            }
        });
        println!("\n");
    }

    let mut total: u64 = 1;
    ranges.iter_mut().for_each(|r| {
        my_ticket.split(',').enumerate().for_each(|(i, v)| {
            let v = v.trim().parse::<u64>().expect("Wanted a number");
            if re_dptr.is_match(&r.0) && i == r.6.into() {
                total *= v;
                println!("{}: {}, TOTAL: {}", &r.0, v, total);
            }
        });
    });
}
