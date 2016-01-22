use std::io;
use std::io::prelude::*;
use std::fs::File;


const EULER: [&'static str; 1] = [
    "euler/problem13.txt"
];

pub fn resolve() {
    let filename = EULER[0];
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
}