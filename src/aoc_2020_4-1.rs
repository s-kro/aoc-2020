/* Advent of Code 2020 Day 4 Puzzle 1 */

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/aoc_2020_4.dat";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut record = String::new();
    let re = Regex::new(r"^\s*$").unwrap();
    let mut i: u32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if !re.is_match(&line.as_str()) {
            record.push_str(" ");
            record.push_str(line.as_str());
        } else {
            println!("{}", record);
            let mut passport = HashMap::new();

            for field in record.split_ascii_whitespace() {
                //println!("{}", field);
                let v: Vec<&str> = field.split(':').collect();
                //println!("l: {} r: {}", v[0], v[1]);
                passport.insert(v[0], v[1]);
            }
            let mut j: u32 = 0;
            for f in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"] {
                //             println!("{}\t{}", f, "g");
                if passport.get(f) == None && f != &"cid" {
                    j = j + 1;

                    println!("{}\t{} {}", f, "Error", j);
                }
            }
            if j == 0 {
                i += 1;
            }
            println!("Good Passports: {}", i);

            // "ecl\t{:?}",
            // match passport.get("ecl") {
            //     None => "No eye colour!",
            //     Some(ref x) => x,
            // }
            //    );
            record.clear();
        }
    }
}

// #[derive(Hash)]
// struct Passport {
//     byr: u32,    // (Birth Year)
//     iyr: u32,    // (Issue Year)
//     eyr: u32,    // (Expiration Year)
//     hgt: String, // (Height)
//     hcl: String, // (Hair Color)
//     ecl: String, // (Eye Color)
//     pid: u64,    // (Passport ID)
//     cid: String, // (Country ID)
// }

//fn 🧝‍(r: &str) -> bool { \u{1f9dd}
//let mut passport: HashMap<String, String> = HashMap::new();
// struct Passport<'a> {
//     record: HashMap<&'a str, &'a str>,
// }

// impl HashMap<&str, &str> {
//     fn new(&self, r: &str) -> Self {
//         for field in r.split_ascii_whitespace() {
//             //println!("{}", field);
//             let mut v: Vec<&str> = field.split(':').collect();
//             //println!("l: {} r: {}", v[0], v[1]);
//             self.record.insert(v[0], v[1]);
//         }
//     }
//     fn is_valid(&self) -> bool {}
// }
// //
//true
//     }
// }

//impl Passport<'_> {
//impl passport(r: &str) -> passport {
//    let mut pp: HashMap<&str, &str> = HashMap::new();
//  for field in r.split_ascii_whitespace() {
//println!("{}", field);
//    pp: Vec<&str> = field.split(':').collect();
//println!("l: {} r: {}", v[0], v[1]);
//  pp.insert(v[0], v[1]);
//let mut v: Vec<&str>;
//  let mut rr: HashMap<&str, &str>;

//    fn new(&self, r: &str) -> Self {
//         for field in r.split_ascii_whitespace() {
//             //println!("{}", field);
//             let mut v: Vec<&str> = field.split(':').collect();
//             //println!("l: {} r: {}", v[0], v[1]);
//             self.record.insert(v[0], v[1]);
//         }
//         self
//         //   Passport { record(self.record) }
//     }
//     fn is_valid(&self) -> bool {
//         true
//     }
// }

// fn def() -> Passport {
//     Passport {
//         byr: 0000,            // (Birth Year)
//         iyr: 0000,            // (Issue Year)
//         eyr: 0000,            // (Expiration Year)
//         hgt: "".to_string(),  // (Height)
//         hcl: "".to_string(),  // (Hair Color)
//         ecl: "6".to_string(), // (Eye Color)
//         pid: 00000000,        // (Passport ID)
//         cid: "".to_string(),  // (Country ID)
//     }
// }
// pub fn new(r: &str) -> Passport {
//     for field in r.split_ascii_whitespace() {
//         println!("{}", field);
//     }
// }
//}
// fn elf(r: &str) -> bool {
//     for field in r.split_ascii_whitespace() {
//         println!("{}", field);
//     }
//     true
//}
