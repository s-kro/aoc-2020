/* Advent of Code 2020 Day 7 Puzzle 2 */

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

    let mut vec: Vec<Vec<(u32, String)>> = Vec::new();
    vec.push(vec![(1, "shiny gold".to_string())]); // start us off!
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
                    // println!("No bags inside");
                } else {
                    assert!(re_insidebags.is_match(&bag[2]));

                    for outer_bag in vec.clone()[bag_level - 1].iter() {
                        if &bag[1] == outer_bag.1 {
                            println!("{} contains {}", &bag[1], &bag[2]);
                            for bag_in_bag in re_insidebags.captures_iter(&bag[2]) {
                                let u: u32 = bag_in_bag[1].trim().parse().expect("Wanted a number");
                                println!(
				    
                                    "{} {} {}",
                                    outer_bag.0 * u,
                                    &bag_in_bag[1],
                                    &bag_in_bag[2]
                                );
                                bags_this_level += outer_bag.0 * u;
                                vec[bag_level].push((outer_bag.0 * u, bag_in_bag[2].to_string()));
                            }
                        }
                    }
                }
            }
        }
        println!(
            "{} bags this level are inside a shiny gold bag",
            bags_this_level
        );

        if bags_this_level == 0 {
            break;
        }
        i += bags_this_level;
    }
    println!("{} total bags are inside a shiny gold bag", i);
}
