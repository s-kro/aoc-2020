/* Advent of Code 2020 Day 21 Puzzle 2 */

use regex::Regex;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_21.dat";

    let re = Regex::new(r"^((?:\w+| )+) \(contains ((?:\w+|, )+)\)$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file); // BTree to keep keys in alphabetical
    let mut food: BTreeMap<String, Vec<String>> = BTreeMap::new(); // order

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));

        //println!("{}\t{}", index, &line);
        for ingd in re.captures_iter(&line) {
            for a in ingd[2].split(", ") {
                if food.contains_key(a) {
                    println!("{} already in hash", a);
                    food.get_mut(a)
                        .unwrap() // only keep the ovelapping ingredients
                        .retain(|ref d| ingd[1].contains(d.as_str()));
                } else {
                    println!("{} not in hash", a);
                    food.insert(
                        a.to_string(),
                        ingd[1].split_whitespace().map(|s| s.to_string()).collect(),
                    );
                }
            }
        }
    }

    loop {
        let mut a: Vec<String> = vec![]; // I know that these are lousy names, but I was trying so many
        let mut i: Vec<String> = vec![]; //  things to get this to work that I got sick of tying to
        let mut all_sorted = true; //        think of new ones
        for f in food.keys() {
            a.push(f.to_string().clone());
            if food.get(f).unwrap().len() == 1 {
                for q in food.get(f).unwrap() {
                    i.push(q.to_string());
                }
            }
        }
        for g in &a {
            // println!("{}, looking for: {:?}", g, a);
            if food.get(g).unwrap().len() > 1 {
                food.get_mut(g).unwrap().retain(|ref d| !i.contains(d));
                all_sorted = false;
            }
        }
        if all_sorted {
            break;
        }
    }

    let mut s: String = "".to_string();
    for f in food.keys() {
        for z in food.get(f).unwrap() {
            s.push_str(z);
        }
        s.push(',');
    }
    s.pop();
    println!("{}", s);
}
