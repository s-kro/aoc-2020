// Advent of Code 2020 Day 20 Puzzle 1

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn aoc_2020_20() {
    let filename = "src/aoc_2020_20.dat";

    let re_num = Regex::new(r"^Tile (\d{4}):$").unwrap();
    let re_tile = Regex::new(r"^([.|#]{10})$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut jigsaw: HashMap<u16, Vec<Vec<char>>> = HashMap::new();
    let mut tile_no = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if re_num.is_match(&line) {
            // println!("{}", &line);
            re_num.captures(&line).into_iter().for_each(|t| {
                tile_no = t[1].trim().parse::<u16>().expect("Wanted a number");
                jigsaw.insert(tile_no, vec![]);
            });
        } else if re_tile.is_match(&line) {
            // println!("{}", &line);
            jigsaw
                .get_mut(&tile_no)
                .unwrap()
                .push(line.chars().collect());
        }
    }
    let mut corner_prod: u64 = 1;

    for (no_1, tile_1) in jigsaw.iter() {
        let tp_1 = tile_1[0].clone();
        let mut bt_1 = tile_1[9].clone();
        let mut rs_1 = vec![];
        let mut ls_1 = vec![];
        for t in tile_1 {
            rs_1.push(t[0]);
            ls_1.push(t[9]);
        }
        bt_1.reverse();
        ls_1.reverse();
        let mut side_match = 0;

        for (no_2, tile_2) in jigsaw.iter() {
            let mut tp_2 = tile_2[0].clone();
            let bt_2 = tile_2[9].clone();
            let mut rs_2 = vec![];
            let mut ls_2 = vec![];
            tp_2.reverse();
            rs_2.reverse();
            for t in tile_2 {
                rs_2.push(t[0]);
                ls_2.push(t[9]);
            }
            if no_1 != no_2 {
                for (i, tst) in [tp_1.clone(), rs_1.clone(), bt_1.clone(), ls_1.clone()]
                    .iter_mut()
                    .enumerate()
                {
                    if tst == &tp_2 || tst == &rs_2 || tst == &bt_2 || tst == &ls_2 {
                        side_match += 1;
                        println!("{}, {} identical, side: {}", no_1, no_2, i);
                    } else {
                        tst.reverse();
                        if tst == &tp_2 || tst == &rs_2 || tst == &bt_2 || tst == &ls_2 {
                            side_match += 1;
                            println!("Reverse {}, {} identical, side: {}", no_1, no_2, i);
                        }
                    }
                }
            }
        }

        println!("Matching sides: {}\n", side_match);
        if side_match == 2 {
            corner_prod *= *no_1 as u64;
        }
    }
    println!("Corner product: {}\n", corner_prod);
}
