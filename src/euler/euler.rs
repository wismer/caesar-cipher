use std::io;
use std::io::prelude::*;
use std::fs::File;


const EULER: [&'static str; 1] = [
    "euler/problem13.txt"
];

pub fn resolve(problem: usize) {
    let filename = EULER[problem];
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let mut total_number = 0u64;
    for line in contents.lines() {
        let numbers: Vec<char> = line.chars().skip(10).collect();
        let mut remainder_val = numbers.iter().fold(0, |acc, &c| {
            let n = c.to_digit(16);
            match n {
                Some(i) => i + acc,
                None    => acc + 0
            }
        });
        println!("{}", remainder_val);
        // {
        //     calculate_remainder(&mut remainder_val, );
        // }
        // total_number += n;
    }

    println!("{}", total_number.to_string())
    // split into lines
    // iterate over the characters
    // return the sum
    // continue to next line; etc.
}

fn calculate_remainder(remainder: &mut i32) {

}

fn line_total(line: &str) -> u64 {
    line.parse::<u64>().unwrap()
}