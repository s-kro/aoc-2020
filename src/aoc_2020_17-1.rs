/* Advent of Code 2020 Day 17 Puzzle 1 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_17.dat";

    let re = Regex::new(r"^(#|\.){8}$").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut c_cube: [Vec<Vec<Vec<char>>>; 2] = [vec![vec![]], vec![vec![]]];

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        assert!(re.is_match(&line));
        c_cube[0][0].push(line.chars().collect());
        c_cube[1][0].push(line.chars().collect());
        println!("{}\t{}", index, &line);
    }
    println!(
        "z: {}, y: {}, x: {}",
        c_cube[0].len(),
        c_cube[0][0].len(),
        c_cube[0][0][0].len()
    );

    for i in 1..=6 {
        let mut active_cubes = 0; // two copies of the cube, we need one for reading and a separate
        let mut xl = 0; //  one for writing  because all the cube changes happen at once,
        let mut yl = 0; //  so we alternate depending on wheather the loop counter is o or e
        for j in 0..=1 {
            for (z, layer) in c_cube[j].clone().iter_mut().enumerate() {
                for (y, _row) in layer.iter().enumerate() {
                    c_cube[j][z][y].insert(0, '.');
                    c_cube[j][z][y].push('.');
                    xl = c_cube[j][0][0].len();
                }
                c_cube[j][z].insert(0, vec!['.'; xl]);
                c_cube[j][z].push(vec!['.'; xl]);
                yl = c_cube[j][0].len();
            }
            c_cube[j].insert(0, vec![vec!['.'; xl]; yl]);
            c_cube[j].push(vec![vec!['.'; xl]; yl]);

            println!(
                "i: {}, j: {}, z: {}, y: {}, x: {}, xl: {}, yl: {}",
                i,
                j,
                c_cube[j].len(),
                c_cube[j][0].len(),
                c_cube[j][0][0].len(),
                xl,
                yl
            );
        }

        let zl = c_cube[i % 2].len() - 1;
        for (z, layer) in c_cube[i % 2].clone().iter_mut().enumerate() {
            let yl = layer.len() - 1;
            for (y, row) in layer.iter().enumerate() {
                let xl = row.len() - 1;
                for (x, cube) in row.iter().enumerate() {
                    let mut cube_mates = 0;
                    for cz in -1i8..=1 {
                        for cy in -1i8..=1 {
                            for cx in -1i8..=1 {
                                if z as i8 + cz < 0
                                    || z as i8 + cz > zl as i8
                                    || y as i8 + cy < 0
                                    || y as i8 + cy > yl as i8
                                    || x as i8 + cx < 0
                                    || x as i8 + cx > xl as i8
                                    || cz == 0 && cy == 0 && cx == 0
                                {
                                } else if c_cube[i % 2][(z as i8 + cz) as usize]
                                    [(y as i8 + cy) as usize]
                                    [(x as i8 + cx) as usize]
                                    == '#'
                                {
                                    cube_mates += 1;
                                } else if c_cube[i % 2][(z as i8 + cz) as usize]
                                    [(y as i8 + cy) as usize]
                                    [(x as i8 + cx) as usize]
                                    == '.'
                                {
                                }
                            }
                        }
                    }

                    if cube == &'#' {
                        if cube_mates == 2 || cube_mates == 3 {
                            c_cube[(i + 1) % 2][z][y][x] = '#';
                            active_cubes += 1;
                        } else {
                            c_cube[(i + 1) % 2][z][y][x] = '.';
                        }
                    } else if cube == &'.' {
                        if cube_mates == 3 {
                            c_cube[(i + 1) % 2][z][y][x] = '#';
                            active_cubes += 1;
                        } else {
                            c_cube[(i + 1) % 2][z][y][x] = '.'
                        }
                    } else {
                        println!("Error");
                    }
                }
            }
            println!("Active Cubes {}", active_cubes);
        }
    }
}
