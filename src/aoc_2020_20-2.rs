// Advent of Code 2020 Day 20 Puzzle 2

use regex::Regex;
use std::collections::HashMap;
use std::convert::TryInto;
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
    const PW: usize = 12;

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
    let mut current_tile = 0;
    let mut row_srt_tile = 0;
    let mut side_orient = 0;

    for (no_1, tile_1) in jigsaw.iter() {
        let tp_1 = tile_1[0].clone();
        let mut bt_1 = tile_1[9].clone();
        let mut rs_1 = vec![];
        let mut ls_1 = vec![];
        for t in tile_1 {
            rs_1.push(t[9]);
            ls_1.push(t[0]);
        }
        bt_1.reverse();
        ls_1.reverse();
        let mut side_match = 0;
        side_orient = 0;

        for (no_2, tile_2) in jigsaw.iter() {
            let mut tp_2 = tile_2[0].clone();
            let bt_2 = tile_2[9].clone();
            let mut rs_2 = vec![];
            let mut ls_2 = vec![];
            for t in tile_2 {
                rs_2.push(t[9]);
                ls_2.push(t[0]);
            }
            tp_2.reverse();
            rs_2.reverse();

            if no_1 != no_2 {
                for (i, tst) in [tp_1.clone(), rs_1.clone(), bt_1.clone(), ls_1.clone()]
                    .iter_mut()
                    .enumerate()
                {
                    if tst == &tp_2 || tst == &rs_2 || tst == &bt_2 || tst == &ls_2 {
                        side_match += 1;
                        side_orient += 2_i32.pow(i.try_into().unwrap());
                        //println!("{}, {} identical, side: {}", no_1, no_2, i);
                    } else {
                        tst.reverse();
                        if tst == &tp_2 || tst == &rs_2 || tst == &bt_2 || tst == &ls_2 {
                            side_match += 1;
                            side_orient += 2_i32.pow(i.try_into().unwrap());
                            //println!("Reverse {}, {} identical, side: {}", no_1, no_2, i);
                        }
                    }
                }
            }
        }
        if side_match == 2 {
            current_tile = *no_1;
            row_srt_tile = *no_1;
            break;
        }
    }
    println!(
        "Start corner tile: {}, side orientation: {}",
        current_tile, side_orient
    );
    println!("Desired orient: {}", 2_i32.pow(1_u32) + 2_i32.pow(2_u32));

    let mut image: [Vec<Vec<char>>; 2] = [vec![vec![]; PW * 8], vec![vec![]; PW * 8]];
    let mut n_tile: Vec<Vec<char>> = vec![vec![]; 10];

    for r in jigsaw[&current_tile].iter().enumerate() {
        n_tile[r.0] = r.1.to_vec();
    }
    if side_orient == 9 || side_orient == 12 {
        n_tile.iter_mut().for_each(|r| {
            r.reverse();
        });
    }
    if side_orient == 3 || side_orient == 9 {
        n_tile.reverse();
    }
    let mut rs_p = vec![];
    let mut bt_p = n_tile[9].to_vec();
    println!("Given");
    for r in jigsaw[&current_tile].iter() {
        println!("{}", r.iter().collect::<String>());
    }
    println!("\nOrientated");
    for r in n_tile.iter_mut().enumerate() {
        rs_p.push(r.1[9]);
        println!("{} {}", r.0, r.1.iter().collect::<String>());
        if let 1..=8 = r.0 {
            r.1.drain(0..1);
            r.1.pop();
            assert_eq![r.1.len(), 8];
            image[0][r.0 - 1].append(r.1);
            image[1][r.0 - 1].append(&mut "12345678".chars().collect());
        }
    }

    let mut tc = 2; // tile count
    let mut tiles: [[u16; PW]; PW] = [[0; PW]; PW];
    tiles[0][0] = current_tile;

    'l: loop {
        for (no, tile) in jigsaw.iter() {
            //println!("Testing: {}", no);
            let tp = tile[0].to_vec();
            let bt = tile[9].to_vec();
            let mut rs = vec![];
            let mut ls = vec![];
            for t in tile {
                ls.push(t[0]);
                rs.push(t[9]);
            }
            let mut t_match = false;
            let mut o_tile: Vec<Vec<char>> = vec![vec![]; 10];

            if no != &current_tile && no != &row_srt_tile {
                for (i, tst) in [tp.to_vec(), rs.to_vec(), bt.to_vec(), ls.to_vec()]
                    .iter_mut()
                    .enumerate()
                {
                    if tst == &rs_p && tc % PW != 1 || tst == &bt_p && tc % PW == 1 {
                        side_orient += 2_i32.pow(i.try_into().unwrap());

                        if tc % PW != 1 {
                            for r in tile.iter().enumerate() {
                                if i % 2 == 0 {
                                    for c in r.1.iter().enumerate() {
                                        o_tile[c.0].push(*c.1);
                                    }
                                } else {
                                    o_tile[r.0] = r.1.to_vec();
                                }
                            }
                            if i == 1 || i == 2 {
                                o_tile.iter_mut().for_each(|r| {
                                    r.reverse();
                                });
                            }
                        } else {
                            row_srt_tile = *no;
                            for r in tile.iter().enumerate() {
                                if i % 2 == 1 {
                                    for c in r.1.iter().enumerate() {
                                        o_tile[c.0].push(*c.1);
                                    }
                                } else {
                                    o_tile[r.0] = r.1.to_vec();
                                }
                            }
                            if i == 1 || i == 2 {
                                o_tile.reverse();
                            }
                        }
                        tiles[(tc - 1) / PW][(tc - 1) % PW] = *no;
                        current_tile = if tc % PW != 0 { *no } else { row_srt_tile };
                        t_match = true;
                        break;
                    } else {
                        tst.reverse();
                        if tst == &rs_p && tc % PW != 1 || tst == &bt_p && tc % PW == 1 {
                            //   side_match += 1;
                            side_orient += 2_i32.pow(i.try_into().unwrap());

                            if tc % PW != 1 {
                                for r in tile.iter().enumerate() {
                                    if i % 2 == 0 {
                                        for c in r.1.iter().enumerate() {
                                            o_tile[c.0].push(*c.1);
                                        }
                                    } else {
                                        o_tile[r.0] = r.1.to_vec();
                                    }
                                }
                                if i == 1 || i == 2 {
                                    o_tile.iter_mut().for_each(|r| {
                                        r.reverse();
                                    });
                                }
                                if i == 0 || i == 1 || i == 2 || i == 3 {
                                    o_tile.reverse();
                                }
                            } else {
                                row_srt_tile = *no;
                                for r in tile.iter().enumerate() {
                                    if i % 2 == 1 {
                                        for c in r.1.iter().enumerate() {
                                            o_tile[c.0].push(*c.1);
                                        }
                                    } else {
                                        o_tile[r.0] = r.1.to_vec();
                                    }
                                }
                                o_tile.iter_mut().for_each(|r| {
                                    r.reverse();
                                });

                                if i == 1 || i == 2 {
                                    o_tile.reverse();
                                }
                            }
                            tiles[(tc - 1) / PW][(tc - 1) % PW] = *no;
                            current_tile = if tc % PW != 0 { *no } else { row_srt_tile };
                            t_match = true;
                            break;
                        }
                    }
                }
            }
            if t_match {
                rs_p.clear();
                for r in o_tile.iter() {
                    rs_p.push(r[9]);
                }
                if tc % PW == 1 {
                    bt_p = o_tile[9].to_vec();
                }

                for r in o_tile.iter_mut().enumerate() {
                    if let 1..=8 = r.0 {
                        r.1.pop();
                        r.1.drain(0..1);
                        assert_eq!(r.1.len(), 8);

                        image[0][((tc - 1) / PW) * 8 + r.0 - 1].append(r.1);
                        image[1][((tc - 1) / PW) * 8 + r.0 - 1]
                            .append(&mut "12345678".chars().collect());
                    } else {
                        r.1.clear();
                    }
                }

                tc += 1
            }

            if tc == PW * PW + 1 {
                for i in image[0].iter().enumerate() {
                    if i.0 % 8 == 0 {
                        //  println!("");
                    }
                    println!("{}", i.1.iter().collect::<String>());
                }
                break 'l;
            }
        }
    }
    for i in 0..PW {
        println!("{:?}", tiles[i]);
    }
    let sea_monster = [
        [0, 18],
        [1, 0],
        [1, 5],
        [1, 6],
        [1, 11],
        [1, 12],
        [1, 17],
        [1, 18],
        [1, 19],
        [2, 1],
        [2, 4],
        [2, 7],
        [2, 10],
        [2, 13],
        [2, 16],
    ];

    for b in 0..=7 {
        let mut sea_monsters = 0;

        for i in 0..PW * 8 - 18 {
            for j in 0..PW * 8 - 1 {
                let mut is_sea_monster = true;
                for sm in sea_monster {
                    if image[b % 2][j + sm[0]][i + sm[1]] == '#'
                        || image[b % 2][j + sm[0]][i + sm[1]] == '0'
                    {
                    } else {
                        is_sea_monster = false;
                        break;
                    }
                }
                if is_sea_monster {
                    for sm in sea_monster {
                        image[b % 2][j + sm[0]][i + sm[1]] = 'O';
                    }
                    sea_monsters += 1;
                }
            }
        }
        let mut waves = 0;
        for i in 0..PW * 8 {
            for j in 0..PW * 8 {
                if image[b % 2][j][i] == '#' {
                    waves += 1;
                }
            }
        }
        println!(
            "\nb:{}, Sea Monsters: {}, Waves: {}",
            b, sea_monsters, waves
        );

        if sea_monsters > 0 {
            for i in image[b % 2].iter().enumerate() {
                if i.0 % 8 == 0 {
                    //  println!("");
                }
                println!("{}", i.1.iter().collect::<String>());
            }
            //break;
        }

        for i in 0..PW * 8 {
            for j in 0..PW * 8 {
                let jm = if b % 4 == 0 { j } else { PW * 8 - 1 - j };
                if image[b % 2][i][jm] == 'O' {
                    image[b % 2][i][jm] = '#';
                }
                image[(b + 1) % 2][j][i] = image[b % 2][i][jm];
            }
        }
    }
}
