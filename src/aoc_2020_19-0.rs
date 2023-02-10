/* Advent of Code 2020 Day 19 Puzzle 1 */

use regex::Regex;
use std::collections::HashMap;
//use std::collections::hash_map::DefaultHasher;
//use std::collections::HashSet;
use std::fs::File;
//use std::iter::repeat;
use std::io::{BufRead, BufReader};

pub fn aoc_2020_19() {
    let r = read_message();
    //let mut i = 42;
    let message: &mut Vec<Vec<char>> = &mut Vec::new(); // vec![vec![]];
    let ms = sub_rule(0, message[0], &r, message);
    //println!("{:?}", ms);
    for m in message {
        println!("{}", m.iter().collect::<String>());
    }
}

fn sub_rule(
    d: usize,
    r: Vec<Vec<u16>>,
    mr: &HashMap<u16, Vec<Vec<u16>>>,
    msg: &mut Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    let mut msg2: Vec<Vec<char>> = Vec::new();
    let szm = mr[&i].len();
    let mut already_doubled = false;
    if d == 0 {
        msg.push(vec![]);
        println!("Initialzing msg");
    }

    let sz = msg.len();

    println!("depth: {}, len {}, szm {}", d, msg.len(), szm);

    for u in mr[&i].iter().enumerate() {
        println!("{:?}", &u.1);
        if d == 0 {
            msg.push(vec![]);
        }

        for v in u.1 {
            //  if d == 0 {
            //     println!("D 0 {:?}", &v);
            //  }
            println!("Processing: {}", &v);
            if *v == 4 || *v == 5 {
                msg2.push(vec![]);
                if *v == 4 {
                    println!("a");
                    for m in msg2.iter_mut().enumerate() {
                        if u.0 == m.0 / sz {
                            m.1.push('a');
                        }
                    }
                    for m in msg.iter_mut().enumerate() {
                        if u.0 == m.0 / sz {
                            m.1.push('a');
                        }
                    }
                } else if *v == 5 {
                    println!("b");
                    msg2.iter_mut().enumerate().for_each(|m| {
                        if u.0 == m.0 / sz {
                            m.1.push('b');
                        }
                    });
                    for m in msg.iter_mut().enumerate() {
                        if u.0 == m.0 / sz {
                            m.1.push('b');
                        }
                    }
                }
            } else {
                println!("depth: {}, len {}, szm {}", d, msg.len(), szm);
                if szm == 2 && !already_doubled {
                    for i in 0..msg.len() {
                        msg.push(msg[i].to_vec());
                    }
                    already_doubled = true;
                    //     println!("sz: {}", sz);
                }

                //let mut res2 = Vec::new();
                // let sz = msg.len();
                // if u.0 == 1 {
                //     for i in 0..sz {
                //         msg.push(msg[i].to_vec());
                //     }
                // }
                // println!("sz: {}", sz);
                for r in res.iter().enumerate() {
                    if r.0 % 2 == u.0 % 2 {
                        msg2.push(r.1.to_vec());
                    }
                    //msg2.push(r.1.to_vec());
                }
                //println!("Res: {:?}", res);
                //msg2.append(&mut sub_rule(d + 1, *v, &mr, msg));
            }
        }
        msg2 = sub_rule(d + 1, *v, &mr, msg);
    }
    msg2
}

fn read_message() -> HashMap<u16, Vec<Vec<u16>>> {
    let filename = "src/aoc_2020_19-t.dat";

    let re_vm = Regex::new(r"^(\d+): ((\d+) ?(\d+)? ?\|? ?(\d+) ?(\d+)?)$").unwrap();
    let re_ve = Regex::new(r#"^(\d+): "([a|b])"?$"#).unwrap();
    let re_rm = Regex::new(r"^([a|b]+)$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut rules: HashMap<u16, Vec<Vec<u16>>> = HashMap::new();
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
                //player = p[1].trim().parse().expect("Wanted a number");
            }
        } else if re_rm.is_match(&line) {
            for _mg in re_rm.captures_iter(&line) {
                //let c = card[1].trim().parse::<u16>().expect("Wanted a number");
                //println!("message: {}", &mg[1]);
                //hands[player - 1].push(c);
            }
        } else {
            println!("Error: Line not captured {}\t{}", _index, &line);
        }
    }
    rules
}
