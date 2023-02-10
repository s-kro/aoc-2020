/* Advent of Code 2020 Day 22 Puzzle 2 */

use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs::File;
//use std::iter::repeat;
use std::{
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
};

pub fn aoc_2020_22() {
    combat(0, read_hands());
}

fn combat(depth: u8, mut h: [Vec<u16>; 2]) -> u8 {
    // println!(
    //     "Recursion depth: {}{}", // a little graph
    //     repeat(' ').take(depth as usize).collect::<String>(),
    //     depth
    // );
    let mut déjà_vu = HashSet::new();
    loop {
        let mut hasher = DefaultHasher::new();
        //    println!("h1: {:?}, \th2: {:?}", h[0], h[1]);
        h.hash(&mut hasher); //https://www.youtube.com/watch?v=0BfIr8Y0CJ4
        if !déjà_vu.insert(hasher.finish()) {
            // println!("Repeated decks");
            return 1; // repeats go to player 1
        }

        let c0 = h[0].remove(0);
        let c1 = h[1].remove(0);

        if c0 as usize <= h[0].len() && c1 as usize <= h[1].len() {
            if combat(
                depth + 1,
                [h[0][0..c0 as usize].to_vec(), h[1][0..c1 as usize].to_vec()],
            ) == 1
            {
                h[0].push(c0); // == 1
                h[0].push(c1);
            } else {
                h[1].push(c1); // == 2
                h[1].push(c0);
            }
        } else if c0 > c1 {
            h[0].push(c0);
            h[0].push(c1);
        } else if c1 > c0 {
            h[1].push(c1);
            h[1].push(c0);
        } else {
            unreachable!(); // cards were equal
        }

        if h[0].is_empty() || h[1].is_empty() {
            let winner = if h[1].is_empty() { 1 } else { 2 };
            if depth == 0 {
                let mut score = 0; // we only care about the score for the top recursion level
                h[winner - 1].iter().rev().enumerate().for_each(|(j, c)| {
                    score += c * (j + 1) as u16;
                });
                println!(" Winner: {}, Score: {}", winner, score);
            }
            return winner as u8;
        }
    }
}

fn read_hands() -> [Vec<u16>; 2] {
    let filename = "src/aoc_2020_22.dat";

    let re_p = Regex::new(r"^Player (\d):$").unwrap();
    let re_c = Regex::new(r"^(\d+)$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut hands: [Vec<u16>; 2] = [vec![], vec![]];
    let mut player = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  // println!("{}\t{}", index, &line);
        if re_p.is_match(&line) {
            for p in re_p.captures_iter(&line) {
                //                println!("player: {}", &p[1]);
                player = p[1].trim().parse().expect("Wanted a number");
            }
        } else if re_c.is_match(&line) {
            for card in re_c.captures_iter(&line) {
                let c = card[1].trim().parse::<u16>().expect("Wanted a number");
                //                println!("card: {}", &card[1]);
                hands[player - 1].push(c);
            }
        }
    }
    hands
}
