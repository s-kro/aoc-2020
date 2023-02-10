/* Advent of Code 2020 Day 19 Puzzle 1 */

use regex::Regex;
use std::collections::HashMap;
//use std::collections::hash_map::DefaultHasher;
//use std::collections::HashSet;
use std::fs::File;
//use std::iter::repeat;
use std::io::{BufRead, BufReader};

pub fn aoc_2020_19() {
    let (rules, letters, rm) = read_message();
    //let mut i = 42;
    let mut m: [Vec<Vec<u16>>; 2] = [Vec::new(), Vec::new()]; // vec![vec![]];
    let mut i = 0;
    for rule in rules[&0].clone() {
        // println!("{:?}", rule);
        m[i].push(rule);
    }
    for i in 0..9 {
        //let tmp: &mut Vec<Vec<u16>> = &mut vec![vec![]];
        let mut done = true;
        for mg in m[i % 2].clone() {
            let tmp: &mut Vec<Vec<u16>> = &mut vec![vec![]];
            for v in mg {
                if v != 116 && v != 131 {
                    let tl = tmp.len();
                    //println!("tmp size in: {} rules: {}", tmp.len(), rules[&v].len());
                    //for t in tmp.clone() {
                    //    println!("{:?}", t.to_vec());
                    //}
                    for _g in 0..rules[&v].len() - 1 {
                        for t in 0..tl {
                            //     println!("g: {}, t: {}", _g, t);
                            tmp.push(tmp[t].to_vec());
                        }
                    }

                    //   println!("tmp size out: {}", tmp.len());
                    for t in tmp.clone() {
                        //      println!("{:?}", t.to_vec());
                    }
                }
                if v == 116 || v == 131 {
                    for t in tmp.iter_mut() {
                        t.push(v);
                    }
                } else {
                    done = false;
                    let sz = tmp.len();
                    for t in tmp.iter_mut().enumerate() {
                        //for rule in rules[&v].iter() {
                        let mf = rules[&v].len();
                        let id = (t.0 * mf) / sz;
                        //if rules[&v].len() == 1 && id == 1 {
                        //    println!("ERROR: both equal");
                        //}
                        t.1.append(&mut rules[&v][id].to_vec());
                        //}
                    }
                }
            }
            //println!("i: {}", i);
            //for t in tmp.clone() {
            //         println!("{:?}", t.to_vec());
            //}
            //	    println!("Checking for Dup");
            for t in tmp {
                // let mut dup = false;
                // for u in &m[(i + 1) % 2] {
                //     if u == t {
                //         println!("Dup");
                //         dup = true;
                //     }
                // }
                // if !dup {
                m[(i + 1) % 2].push(t.to_vec());
                // }
            }
            //println!("Finished Checking for Dup");

            m[i % 2].clear();
        }
        if done {
            println!("Break");
            break;
        }
    }
    //println!("{:?}", ms);
    let mut finished_valid: Vec<String> = Vec::new();
    let mut vmc = 0;
    println!("\n Exit");
    for m1 in m {
        for m2 in m1 {
            let mut tmp: String = "".to_string();
            //println!("{:?}", m2);
            for m3 in m2 {
                tmp.push(letters[&m3]);
            }
            finished_valid.push(tmp.to_string());
            for rm in &rm {
                if tmp == *rm {
                    vmc += 1;
                }
            }
        }
    }
    println!("Matches: {}", vmc);
    // for fv in finished_valid {
    //     println!("{}", fv);
    //}
    for _u in rules[&0].iter().enumerate() {
        // for v in u.1.iter_mut().enumerate() {
        //  r[&0].retain(f);
        // }
    }
}

fn read_message() -> (HashMap<u16, Vec<Vec<u16>>>, HashMap<u16, char>, Vec<String>) {
    let filename = "src/aoc_2020_19.dat";

    let re_vm = Regex::new(r"^(\d+): (\d+(?: \d+)?(?: \| \d+)?(?: \d+)?)$").unwrap();
    let re_ve = Regex::new(r#"^(\d+): "([a|b])"?$"#).unwrap();
    let re_rm = Regex::new(r"^([a|b]+)$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut rules: HashMap<u16, Vec<Vec<u16>>> = HashMap::new();
    let mut letters: HashMap<u16, char> = HashMap::new();
    let mut rec_messages = Vec::new();
    //let mut player = 0;

    for (_index, line) in reader.lines().enumerate() {
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
                //println!("{:?}", &sub_rules);
                rules.insert(k, sub_rules);
            }
        } else if re_ve.is_match(&line) {
            for ve in re_ve.captures_iter(&line) {
                println!("rule: {}, {} {}", &ve[0], &ve[1], &ve[2]);
                let c = ve[1].trim().parse::<u16>().expect("Wanted a number");
                letters.insert(c, ve[2].chars().last().unwrap());
                println!("finish chars: {:?}", letters);
                //player = p[1].trim().parse().expect("Wanted a number");
            }
        } else if re_rm.is_match(&line) {
            for mg in re_rm.captures_iter(&line) {
                rec_messages.push(mg[1].to_string());
                //let c = mg[1].trim().parse::<u16>().expect("Wanted a number");
                //letters.insert(c, mg[2].chars().last().unwrap());
                //println!("finish chars: {:?}", letters);
                ////hands[player - 1].push(c);
            }
        } else {
            println!("Error: Line not captured {}\t{}", _index, &line);
        }
    }
    (rules, letters, rec_messages)
}
