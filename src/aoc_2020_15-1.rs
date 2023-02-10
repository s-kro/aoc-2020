/* Advent of Code 2020 Day 15 Puzzle 1 */

//use regex::Regex;
//use std::fs::File;
//use std::io::{BufRead, BufReader};

fn main() {
    //let filename = "src/aoc_2020_15.dat";
    let input = vec![0, 14, 1, 3, 7, 9];
    let mut s = vec![];
    //let file = File::open(filename).unwrap();
    //let reader = BufReader::new(file);

    for t in 0..2020 {
        println!("Turn {}", t + 1);
        if t < input.len() {
            s.push(input[t]);
        } else {
            let ln = s[t - 1];
            //  println!("ln: {}", ln);
            let mut first_time = true;
            for b in (0..t - 1).rev() {
                // println!("b: {}", b);
                if ln == s[b] {
                    //println!("Match ln: {}, b: {}", ln, b);
                    println!("Say: {}", t - b - 1);
                    s.push(t - b - 1);
                    first_time = false;
                    break;
                }
            }
            if first_time {
                s.push(0);
                println!("Say: {}", 0);
            }
        }
        println!("\n");
        // println!("{}\t{}", index, &line);
    }
}
