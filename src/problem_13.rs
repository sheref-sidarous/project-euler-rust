use std::fs::File;
use std::io::{self, BufRead};
use crate::bigint::BigInt;

pub fn solution() {
    let file = File::open("res/problem_13.txt").unwrap();

    let mut acc = BigInt::new(0u64);
    
    for line in io::BufReader::new(file).lines() {
        let number = BigInt::from_str(line.unwrap().as_str());
        //first_10_digits.drain(0..40);
        acc = &acc + &number;
    }
    
    println!("{}", acc.into_str());

}