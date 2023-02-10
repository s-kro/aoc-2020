/* Advent of Code 2015 Day 25 Puzzle 1 */

fn aoc_2015_25() {
    let c: u64 = 20151125; // code
    let m: u64 = 252533; // multiplicand
    let d: u64 = 33554393; // divisor
    let mut pk: u64 = 1; // public key
    let mut sn: u64 = 7; // subject number
    let mut ls: u32 = 0; // loop size
    let mut k_ls = 0;
    let mut d_ls = 0;
    println!("mult: {}\tdiv: {}", m, d);

    let mut i = 0;
    loop {
        i += 1;
        c = c * m % d;

        println!("i: {} code: {}", i, c);
        if i == 25 {
            break;
        }
    }
}
