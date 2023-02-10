/* Advent of Code 2020 Day 25 Puzzle 1 */

fn main() {
    let k: u64 = 3248366; // key
    let d: u64 = 4738476; // door
    let mut pk: u64 = 1; // public key
    let mut sn: u64 = 7; // subject number
    let mut ls: u32 = 0; // loop size
    let mut k_ls = 0;
    let mut d_ls = 0;
    println!("key: {}\tdoor: {}", k, d);

    loop {
        pk *= sn;
        pk %= 20201227;
        ls += 1;
        //println!("public key: {}\tls: {}", pk, ls);
        if pk == k {
            k_ls = ls;
        } else if pk == d {
            d_ls = ls;
        }
        if k_ls != 0 && d_ls != 0 {
            break;
        }
    }
    println!("door ls: {}\tkey ls: {}", d_ls, k_ls);

    sn = k;
    pk = 1;
    ls = 0;
    loop {
        pk *= sn;
        pk %= 20201227;
        ls += 1;
        //println!("public key: {}\tls: {}", pk, ls);
        if ls == d_ls {
            println!("door encryption key: {}\tkey ls: {}", pk, k_ls);
            break;
        }
    }
}
