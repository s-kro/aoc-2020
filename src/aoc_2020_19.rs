/* Advent of Code 2020 Day 19 Puzzle 2 */

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn aoc_2020_19() {
    let (rules, letters, rm) = read_message();
    let mut finished_valid: [Vec<String>; 2] = [Vec::new(), Vec::new()];

    for j in [42, 31] {
        let mut m: [Vec<Vec<u16>>; 2] = [Vec::new(), Vec::new()];
        //let m: &mut Vec<Vec<u16>> = &mut Vec::new();
        for rule in rules[&j].clone() {
            // println!("{:?}", rule);
            m[0].push(rule);
        }
        for i in 0..9 {
            let mut done = true; // flop back and forth between buffers to avoid hidden
            for mg in m[i % 2].clone() {
                let tmp: &mut Vec<Vec<u16>> = &mut vec![vec![]]; // ... & and borrow issues
                for v in mg {
                    if v != 116 && v != 131 {
                        let tl = tmp.len(); // if there is a branch | in the rule
                        for _ in 1..rules[&v].len() {
                            for t in 0..tl {
                                tmp.push(tmp[t].to_vec()); // ... duplicate every possibilty
                            } //      ... start from 1 because we already have a copy
                        }
                    }
                    if v == 116 || v == 131 {
                        for t in tmp.iter_mut() {
                            t.push(v);
                        }
                    } else {
                        done = false;
                        let sz = tmp.len(); // ... then append the new rule,
                        for t in tmp.iter_mut().enumerate() {
                            let mf = rules[&v].len(); // ... first half of the new branch
                            let id = (t.0 * mf) / sz; // to the first half of the possibilties
                            t.1.append(&mut rules[&v][id].to_vec()); // ... and the second half to
                        } //                                            ... the second half
                    }
                }
                //m[0 % 2].clear();
                for t in tmp {
                    m[(i + 1) % 2].push(t.to_vec());
                }
                m[i % 2].clear();
            }
            if done {
                println!("Break");
                break;
            }
        }
        // Generate the strings of letter from the finish rule sets
        let fvi = if j == 42 { 0 } else { 1 };
        println!("\n Exit");
        for m1 in m.iter() {
            for m2 in m1 {
                let mut tmp: String = "".to_string();
                //println!("{:?}", m2);
                for m3 in m2 {
                    tmp.push(letters[&m3]);
                }
                finished_valid[fvi].push(tmp.to_string());
            }
        }
    }
    // let mut matches = 0; // check for overlap in the rule sets
    // for fv0 in finished_valid[0].clone() {
    //     for fv1 in finished_valid[1].clone() {
    //         if fv0 == fv1 {
    //             matches += 1;
    //             println!("Matches: {} {}", matches, fv0);
    //         }
    //     }
    // }

    let ss_len = finished_valid[0][0].len(); // rulesets 42 and 11 are
    assert_eq!(ss_len, finished_valid[1][0].len()); // ... equal length

    let mut vmc = 0;
    for rm in &rm {
        let mut chc = rm.len() / ss_len;
        // println!("Received Message: {}, chunk count: {}", rm, chc);
        let j = [0_usize, 1];
        let mut j_index = 0;
        let mut j_matches = [0; 3];
        'next_msg: for rmc in rm
            .chars()
            .collect::<Vec<char>>()
            .chunks(ss_len)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
        {
            let mut fvc_match = false;
            for fv in finished_valid[j[j_index]].clone() {
                if fv == rmc {
                    chc -= 1;
                    //println!("Match, chc: {}", chc);
                    j_matches[j_index] += 1;
                    fvc_match = true;
                    break;
                }
            }
            if !fvc_match {
                if j_index == 1 {
                    break 'next_msg;
                } else {
                    j_index += 1;
                    //let mut fvc_match = false;
                    for fv in finished_valid[j[j_index]].clone() {
                        if fv == rmc {
                            chc -= 1;
                            j_matches[j_index] += 1;
                            // println!("Match 2nd try, chc: {}", chc);
                            //fvc_match = true;
                            break;
                        }
                    }
                    //   break 'retry;
                }
            }
            if chc == 0 && j_index == 1 {
                if j_matches[0] >= 1 && j_matches[1] >= 1 && j_matches[0] > j_matches[1] {
                    vmc += 1;
                    // println!("    Match String");
                }
            }
        }
    }
    println!("Matches: {}", vmc);
}

fn read_message() -> (HashMap<u16, Vec<Vec<u16>>>, HashMap<u16, char>, Vec<String>) {
    let filename = "src/aoc_2020_19.dat";

    let re_vm =
        Regex::new(r"^(\d+): (\d+(?: \d+)?(?: \| \d+)?(?: \d+)?(?: \d+)?(?: \d+)?)$").unwrap();
    let re_ve = Regex::new(r#"^(\d+): "([a|b])"?$"#).unwrap();
    let re_rm = Regex::new(r"^([a|b]+)$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut rules: HashMap<u16, Vec<Vec<u16>>> = HashMap::new();
    let mut letters: HashMap<u16, char> = HashMap::new();
    let mut rec_messages = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  // println!("{}\t{}", index, &line);
        if re_vm.is_match(&line) {
            for vm in re_vm.captures_iter(&line) {
                let k = vm[1].trim().parse().expect("Wanted a number");
                rules.insert(k, vec![]);
                //println!("sub rule: {}, {}", k, &vm[2]);
                let mut sub_rules: Vec<Vec<u16>> = vec![vec![]];

                let mut i = 0;
                for v in vm[2].split_whitespace() {
                    if v == "|" {
                        sub_rules.push(vec![]);
                        i = 1;
                    } else {
                        sub_rules[i].push(v.trim().parse().expect("Wantesd a number"));
                    }
                }
                rules.insert(k, sub_rules);
            }
        } else if re_ve.is_match(&line) {
            for ve in re_ve.captures_iter(&line) {
                println!("rule: {}, {} {}", &ve[0], &ve[1], &ve[2]);
                let c = ve[1].trim().parse::<u16>().expect("Wanted a number");
                letters.insert(c, ve[2].chars().last().unwrap());
                println!("finish chars: {:?}", letters);
            }
        } else if re_rm.is_match(&line) {
            for mg in re_rm.captures_iter(&line) {
                rec_messages.push(mg[1].to_string());
            }
        } else {
            println!("Error: Line not captured {}\t{}", index, &line);
        }
    }
    (rules, letters, rec_messages)
}
