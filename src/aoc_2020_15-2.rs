/* Advent of Code 2020 Day 15 Puzzle 2 */


// This took almost 48 hours to run!! (Even
// with release mode level 3) It gave the
// correct answer, though. Day 15 Puzzle 3
// gives some much better methods
fn main() {
    let input = vec![0, 14, 1, 3, 7, 9];
    let mut s = vec![];

    for t in 0..30000000 {
        if t < input.len() {
            s.push(input[t]);
        } else {
            let ln = s[t - 1];
            let mut first_time = true;
            for b in (0..t - 1).rev() {
                if ln == s[b] {
                    s.push(t - b - 1);
                    first_time = false;
                    break;
                }
            }
            if first_time {
                s.push(0);
            }
        }
        if (t + 1) % 100000 == 0 {
            println!("\nTurn {}, Say: {}", t + 1, s[t]);
        }
    }
}
