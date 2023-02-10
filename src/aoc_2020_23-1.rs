/* Advent of Code 2020 Day 23 Puzzle 1 */

pub fn aoc_2020_23() {
    const RING_SZ: usize = 9;
    let mut cup_circle = [1, 9, 3, 4, 6, 7, 2, 5, 8];
    const ROUNDS: i32 = 100;

    let mut curremt_cup = 0;

    for _i in 1..=ROUNDS {
        //println!("\n-- move {} --\ncups: {:?}", i, cup_circle);
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
        //println!("Pick up: {:?}, destination: {}", pick_up, destiation_label);

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
    println!("\n-- final --\ncups: {:?}", cup_circle);
    let mut str_strt = 0;
    let mut cup_final: String = r#""#.to_string();
    for i in 0..RING_SZ {
        if cup_circle[i] == 1 {
            str_strt = i + 1;
            break;
        }
    }
    for i in 0..RING_SZ - 1 {
        cup_final += &cup_circle[(str_strt + i) % RING_SZ].to_string();
    }
    println!("\nfinal form: {}", cup_final);
}
