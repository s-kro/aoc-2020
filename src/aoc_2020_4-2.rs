/* Advent of Code 2020 Day 4 Puzzle 2 */

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
                let v: Vec<&str> = field.split(':').collect();
                passport.insert(v[0], v[1]);
            }
            let mut j: u32 = 0;
            for f in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"] {
                if passport.get(f) == None && f != &"cid" {
                    j += 1;
                    println!("{}\t{} {}", f, "Error", j);
                } else if f == &"byr" || f == &"iyr" || f == &"eyr" {
                    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
                    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.

                    let re = Regex::new(r"^(\d{4})$").unwrap();
                    assert!(re.is_match(passport[f]));
                    for cap in re.captures_iter(passport[f]) {
                        let yr: i32 = cap[1].trim().parse().expect("Wanted a number");
                        if f == &"byr" && (yr < 1920 || yr > 2002)
                            || f == &"iyr" && (yr < 2010 || yr > 2020)
                            || f == &"eyr" && (yr < 2020 || yr > 2030)
                        {
                            println!("Error {}: {} out of range", f, &yr);
                            j += 1;
                        }
                    }
                } else if f == &"hgt" {
                    // hgt (Height) - a number followed by either cm or in:
                    //     If cm, the number must be at least 150 and at most 193.
                    //     If in, the number must be at least 59 and at most 76.

                    let re = Regex::new(r"^(\d+)(|in|cm)$").unwrap();
                    assert!(re.is_match(passport[f]));

                    for cap in re.captures_iter(passport[f]) {
                        let ht: i32 = cap[1].trim().parse().expect("Wanted a number");
                        if &cap[2] == "cm" && (ht < 150 || ht > 193)
                            || &cap[2] == "in" && (ht < 59 || ht > 76)
                            || &cap[2] == ""
                        {
                            println!("Error {}: {} {} out of range", f, &ht, &cap[2]);
                            j += 1;
                        }
                    }
                } else if f == &"hcl" {
                    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                    let re = Regex::new(r"^(#[\da-f]{6})$").unwrap();
                    //assert!(re.is_match(passport[f]));

                    if !re.is_match(passport[f]) {
                        println!("Error {}: doesnt match regex", f);
                        j += 1;
                    }
                } else if f == &"ecl" {
                    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                    let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
                    //assert!(re.is_match(passport[f]));

                    if !re.is_match(passport[f]) {
                        println!("Error {}: doesnt match regex", f);
                        j += 1;
                    }
                } else if f == &"pid" {
                    // pid (Passport ID) - a nine-digit number, including leading zeroes.
                    let re = Regex::new(r"^(\d{9})$").unwrap();
                    //assert!(re.is_match(passport[f]));

                    if !re.is_match(passport[f]) {
                        println!("Error {}: doesnt match regex", f);
                        j += 1;
                    }
                }
                // cid (Country ID) - ignored, missing or not.
            }
            if j == 0 {
                i += 1;
            }
            println!("Good Passports: {}", i);
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
