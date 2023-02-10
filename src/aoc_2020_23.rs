/* Advent of Code 2020 Day 23 Puzzle 2 */

//use std::collections::HashMap;
use std::io;
use std::io::Write;
//use std::u32;

pub fn aoc_2020_23() {
    const RING_SZ: usize = 1_000_000;
    let mut cup_circle: [i32; RING_SZ] = [0; RING_SZ];

    let input = vec![1, 9, 3, 4, 6, 7, 2, 5, 8];
    //let input = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    for c in 0..RING_SZ {
        if c < input.len() {
            cup_circle[c] = input[c];
        } else {
            cup_circle[c] = c as i32 + 1;
        }
    }

    const ROUNDS: i32 = 10_000_000;
    let mut curremt_cup = 0;

    for i in 1..=ROUNDS {
        // println!(
        //     "\n-- move {} -- current cup: {}",
        //     i, cup_circle[curremt_cup]
        // );
        let pick_up = [
            cup_circle[(curremt_cup + 1) % RING_SZ],
            cup_circle[(curremt_cup + 2) % RING_SZ],
            cup_circle[(curremt_cup + 3) % RING_SZ],
        ];
        let mut destiation_label = 0;
        for j in 1..=4 {
            destiation_label = if cup_circle[curremt_cup] - j <= 0 {
                cup_circle[curremt_cup] - j + RING_SZ as i32
            } else {
                cup_circle[curremt_cup] - j
            };
            //destiation_label = (cup_circle[curremt_cup] + 9 - j) % 9;
            if destiation_label != pick_up[0]
                && destiation_label != pick_up[1]
                && destiation_label != pick_up[2]
            {
                //println!("destination: {}", destiation_label);
                break;
            }
        }

        if i % 1000 == 0 {
            print!(
                "\rRound: {} Pick up: {:?}, destination: {}",
                i, pick_up, destiation_label
            );
            io::stdout().flush().unwrap();
        }

        for j in 1..=RING_SZ {
            //println!("j: {}, cup: {}", j, cup_circle[(curremt_cup + j) % 9]);
            cup_circle[(curremt_cup + j) % RING_SZ] = cup_circle[(curremt_cup + j + 3) % RING_SZ];

            if cup_circle[(curremt_cup + j + 3) % RING_SZ] == destiation_label {
                //println!("j: {}, dest_lab: {}", j, destiation_label);
                cup_circle[(curremt_cup + j + 1) % RING_SZ] = pick_up[0];
                cup_circle[(curremt_cup + j + 2) % RING_SZ] = pick_up[1];
                cup_circle[(curremt_cup + j + 3) % RING_SZ] = pick_up[2];
                break;
            }
        }
        curremt_cup = (curremt_cup + 1) % RING_SZ;
    }
    // println!("\n-- final --\ncups: {:?}", cup_circle);

    for i in 0..RING_SZ {
        if cup_circle[i] == 1 {
            let tot: u64 = cup_circle[i + 1] as u64 * cup_circle[i + 2] as u64;
            println!("\n\n{}, {}, {}", cup_circle[i + 1], cup_circle[i + 2], tot);
            break;
        }
    }
}
