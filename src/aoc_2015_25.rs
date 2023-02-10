/* Advent of Code 2015 Day 25 Puzzle 1 */
// Enter the code at row 2978, column 3083.

pub fn aoc_2015_25() {
    let mut c: u64 = 20151125; // code
    let m: u64 = 252533; // multiplicand
    let d: u64 = 33554393; // divisor
    let mut col = 1;
    let mut row = 1;
    println!("mult: {}\tdiv: {}", m, d);

    let mut i = 0;
    loop {
        i += 1;
        c = c * m % d;
        if row == 1 {
            row = col + 1;
            col = 1;
        } else {
            row -= 1;
            col += 1;
        }
        if row == 2978 && col == 3083 {
            println!("i: {},\trow: {},\tcol: {},\tcode: {}", i, row, col, c);
            break;
        }
    }
}
