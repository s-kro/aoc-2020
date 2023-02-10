/* Advent of Code 2020 Day 7 Puzzle 1 */

use regex::Regex;
//use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_7.dat";

    let re = Regex::new(r"^(\w+ \w+) bags contain ((?:\d|no).*)\.$").unwrap();
    let re_nobag = Regex::new(r"^no .*$").unwrap();
    let re_insidebags = Regex::new(r"(?:(\d) (\w+ \w+) bag)+").unwrap();

    let mut i: u32 = 0;

    let mut vec: Vec<Vec<String>> = Vec::new();
    vec.push(vec!["shiny gold".to_string()]); // start us off!
    let mut bag_level = 0;
    loop {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        vec.push(Vec::new());
        bag_level += 1;
        let mut bags_this_level = 0;
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            assert!(re.is_match(&line));
            for bag in re.captures_iter(&line) {
                //println!("{} contains {}", &bag[1], &bag[2]);
                if re_nobag.is_match(&bag[2]) {
                    //   println!("No bags inside");
                } else {
                    assert!(re_insidebags.is_match(&bag[2]));
                    for bag_in_bag in re_insidebags.captures_iter(&bag[2]) {
                        //println!("{} {}", &bag_in_bag[1], &bag_in_bag[2]);
                        for inner_bag in vec.clone()[bag_level - 1].iter() {
                            if &bag_in_bag[2] == inner_bag {
                                println!(
                                    "bag level {}, {} contains {}",
                                    bag_level, &bag[1], &inner_bag
                                );

                                let mut dup = false;
                                for (k, lvl) in vec.iter().enumerate() {
                                    for dup_bag in lvl.iter() {
                                        if dup_bag == &bag[1] {
                                            println!("duplicate bag {} at level: {}", dup_bag, k);
                                            dup = true;
                                        }
                                    }
                                }
                                if !dup {
                                    vec[bag_level].push(bag[1].to_string());
                                    bags_this_level += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        println!(
            "{} bags this level contain shiny gold bag(s)",
            bags_this_level
        );

        //      for bag in vec[0].iter() {
        //          println!("{} should say  shiny gold bag(s)", &bag);
        //      }
        //      for bag in vec[1].iter() {
        //         println!("{} contain shiny gold bag(s)", &bag);
        //     }
        if bags_this_level == 0 {
            break;
        }
        i += bags_this_level;
    }
    println!("{} bags (eventually) contain shiny gold bag(s)", i);
}
