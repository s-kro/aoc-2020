/* Advent of Code 2020 Day 24 Puzzle 1 */

use regex::Regex;
//use std::collections::HashSet;
//use std::convert::TryInto;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_24.dat";

    let re = Regex::new(r"^(ne|nw|se|sw|e|w)(\w+|)$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut fin_tile: Vec<[i16; 2]> = Vec::new();
    let mut x_min: i16 = 0;
    let mut x_max: i16 = 0;
    let mut y_min: i16 = 0;
    let mut y_max: i16 = 0;
    let mut pm = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));
        println!("{}\t{}", index, &line);
        let mut pos_tile: [i16; 2] = [0; 2];
        let mut l = line;
        'outer: loop {
            for tiles in re.captures_iter(&l.clone()) {
                //println!("{}\t{}\t{}", index, &tiles[1], &tiles[2]);
                match &tiles[1] {
                    "e" => {
                        //  println!("East");
                        pos_tile[0] -= 2;
                    }
                    "w" => {
                        //   println!("West");
                        pos_tile[0] += 2;
                    }
                    "nw" => {
                        //  println!("North West");
                        pos_tile[0] += 1;
                        pos_tile[1] += 1;
                    }
                    "ne" => {
                        //  println!("North East");
                        pos_tile[0] -= 1;
                        pos_tile[1] += 1;
                    }
                    "sw" => {
                        //  println!("South West");
                        pos_tile[0] += 1;
                        pos_tile[1] -= 1;
                    }
                    "se" => {
                        //  println!("South East");
                        pos_tile[0] -= 1;
                        pos_tile[1] -= 1;
                    }
                    _ => println!("Error"),
                }
                if &tiles[2] == "" {
                    println!("Break x: {}\ty: {}", pos_tile[0], pos_tile[1]);
                    x_min = min(pos_tile[0], x_min);
                    x_max = max(pos_tile[0], x_max);
                    y_min = min(pos_tile[1], y_min);
                    y_max = max(pos_tile[1], y_max);

                    let mut tpm = false; // tile position match

                    for ft in fin_tile.clone() {
                        //println!("Checking Tile Pos Match x: {}\ty {}", ft[0], ft[1]);

                        if pos_tile == ft {
                            println!("Tile Pos Match");

                            tpm = true;
                            pm += 1;
                            break;
                        }
                    }
                    fin_tile.retain(|&ft| ft != pos_tile);
                    if !tpm {
                        println!("No Tile Pos Match");
                        fin_tile.push(pos_tile);
                    }
                    break 'outer;
                }
                l = tiles[2].to_string();
            }
        }
    }
    println!("Matched tiles: {}, Flipped tiles: {}", pm, fin_tile.len());
    println!(
        "x min: {}, x max: {} y min: {}, y max: {}",
        x_min, x_max, y_min, y_max
    );
}
