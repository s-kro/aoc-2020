/* Advent of Code 2020 Day 24 Puzzle 2 */

use regex::Regex;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_24.dat";

    let re = Regex::new(r"^(ne|nw|se|sw|e|w)(\w+|)$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut fin_tile: [Vec<[i16; 2]>; 2] = [vec![], vec![]];
    let mut x_min: i16 = 0; // fin_tile should be renamed black_tile
    let mut x_max: i16 = 0;
    let mut y_min: i16 = 0;
    let mut y_max: i16 = 0;
    let mut pm = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));
        // println!("{}\t{}", index, &line);
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
                    //println!("Break x: {}\ty: {}", pos_tile[0], pos_tile[1]);
                    x_min = min(pos_tile[0], x_min);
                    x_max = max(pos_tile[0], x_max);
                    y_min = min(pos_tile[1], y_min);
                    y_max = max(pos_tile[1], y_max);

                    let mut tpm = false; // tile position match

                    for ft in fin_tile[0].clone() {
                        //println!("Checking Tile Pos Match x: {}\ty {}", ft[0], ft[1]);

                        if pos_tile == ft {
                            //println!("Tile Pos Match");

                            tpm = true;
                            pm += 1;
                            break;
                        }
                    }
                    fin_tile[0].retain(|&ft| ft != pos_tile);
                    if !tpm {
                        //     println!("No Tile Pos Match");
                        fin_tile[0].push(pos_tile);
                    }
                    break 'outer;
                }
                l = tiles[2].to_string();
            }
        }
    }
    println!(
        "Matched tiles: {}, Flipped tiles: {}",
        pm,
        fin_tile[0].len()
    );
    println!(
        "x min: {}, x max: {} y min: {}, y max: {}",
        x_min, x_max, y_min, y_max
    );
    for day in 1..=100 {
        fin_tile[day % 2].clear(); // reset the buffer for today
        for y in y_min - 1..=y_max + 1 {
            for x in x_min - 2..=x_max + 2 {
                if x.abs() % 2 == y.abs() % 2 {
                    let mut bct = 0; // count of surrounding black tiles
                    let mut bt = false; // black tile = true
                    for cy in y - 1..=y + 1 {
                        let r = if y == cy { 2 } else { 1 };
                        for cx in (x - r..=x + r).step_by(2) {
                            // println!("Checking Tile Pos Match cx: {}\tcy {}", cx, cy);
                            for ft in fin_tile[(day - 1) % 2].clone() {
                                if [cx, cy] == ft {
                                    if [cx, cy] == [x, y] {
                                        // println!("\t\tBlack Tile {} {}", x, y);
                                        bt = true; // if this tile is black
                                    } else {
                                        bct += 1; // else the other 6 that are surrounding
                                                  // println!("\t\tSurrounding Black Tile {} {}", cx, cy)
                                    }
                                }
                            }
                        }
                    }
                    // println!("\tBlack Tile {} Surrounding B T {}", bt, bct);
                    if bt && (bct == 0 || bct > 2) {
                        // println!("\tBlack Tile {}, {} flipped to white, Day {}", x, y, day);
                    } else if !bt && bct == 2 {
                        // println!("\tWhite Tile {}, {} flipped to black, Day {}", x, y, day);
                        x_min = min(x, x_min); // expand the 'floor space' as
                        x_max = max(x, x_max); //   required to check new border
                        y_min = min(y, y_min); //   white tiles
                        y_max = max(y, y_max);
                        fin_tile[day % 2].push([x, y]);
                    } else if bt {
                        // println!("\tBlack tile {}, {} stays black, Day {}", x, y, day);
                        fin_tile[day % 2].push([x, y]);
                    }
                }
            }
        }
        println!(
            "x min: {}, x max: {} y min: {}, y max: {}",
            x_min, x_max, y_min, y_max
        ); // this starts to get really slow as the floor area spreads out!!
        println!("Day {}, Black tiles: {}", day, fin_tile[day % 2].len());
    }
}
