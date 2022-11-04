
use crate::bigint::BigInt;

pub fn solution() {
    let x : BigInt = 1.into();
    let y = &x << 1000u64;

    let sum : u64 = y.into_str()
                    .chars()
                    .map(|char| {
                                char.to_digit(10).unwrap() as u64
                            })
                    .sum();

    println!("{}", sum);

}