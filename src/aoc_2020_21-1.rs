/* Advent of Code 2020 Day 21 Puzzle 1 */

use regex::Regex;
use std::collections::HashMap;
//use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_21.dat";

    let re = Regex::new(r"^((?:\w+| )+) \(contains ((?:\w+|, )+)\)$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut food: HashMap<String, Vec<String>> = HashMap::new();
    let mut allergens: Vec<String> = vec![];
    let mut total_ingd: Vec<String> = vec![];

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));

        //println!("{}\t{}", index, &line);
        for ingd in re.captures_iter(&line) {
            for i in ingd[1].split_whitespace() {
                total_ingd.push(i.to_string());
            }
            for a in ingd[2].split(", ") {
                println!("{}", a);
                if food.contains_key(a) {
                    println!("{} already in hash", a);
                    food.get_mut(a)
                        .unwrap()
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
    for f in food.clone().keys() {
        println!(
            "{} => {:#?}, len: {}",
            f,
            food.get(f).unwrap(),
            food.get(f).unwrap().len()
        );
        for a in food.clone().get(f).unwrap() {
            allergens.push(a.to_string());
        }
    }
    //  for a in &allergens {
    //      println!("allergen: {}", a)
    //  }
    allergens.sort();
    allergens.dedup();

    total_ingd.retain(|ref d| !allergens.contains(d));

    println!("Total safe ingred: {}", total_ingd.len());
}
