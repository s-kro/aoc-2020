/* Advent of Code 2020 Day 15 Puzzle 2 */

use std::collections::HashMap;

fn main() {
    let target = 30000000;
    let mut seen = [0, 14, 1, 3, 7, 9]
        .iter()
        .enumerate()
        .map(|(i, &e)| (e, i as u32 + 1))
        .collect::<HashMap<_, _>>();

    println!(
        "{}",
        (7..target).fold(0, |last, i| i - seen.insert(last, i).unwrap_or(i))
    );

    // this was still a million trillion times faster than my original method
    let input = vec![0, 14, 1, 3, 7, 9];
    // let input = vec![0, 3, 6];
    let mut s: HashMap<u32, u32> = HashMap::new();

    let mut last = 0;
    for t in 0..30000000 {
        if t < input.len() {
            s.insert(input[t], t as u32 + 1);
        } else {
            let mut ln: u32 = t as u32 + 1; // default
            if s.contains_key(&last) {
                ln = s[&last];
            }
            //println!("turn: {}, ln: {}, Say: {}", t + 1, ln, last);
            s.insert(last, t as u32 + 1);

            if (t + 1) % 300000 == 0 {
                println!("\nTurn {}, Say: {}", t + 1, last);
            }

            last = t as u32 - ln + 1;
        }
    }
}
