
use crate::primes::Primes;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PrimeFactor {
    prime : u64,
    power : u64,
}
// loop
pub fn prime_facorization(input : u64) -> Vec<PrimeFactor> {
    
    let mut reminder = input;
    let mut facorized_primes = Vec::new();
    
    let prime_list = Primes::new();
    for prime in prime_list {
        let mut power = 0;
        while reminder % prime == 0 {
            reminder = reminder / prime;
            power += 1;
        }
        if power > 0 {
            facorized_primes.push(PrimeFactor { prime: prime, power: power });
        }
        if reminder == 1 {
            break;
        }
    }

    return facorized_primes;
}

pub fn number_of_divisors(input : u64) -> u64 {
    let prime_factors = prime_facorization(input);
    let no_of_divisors = prime_factors.iter().fold(1, 
        | acc, factor | {  acc * (factor.power + 1)} );

    return no_of_divisors;
}

// recursive 
pub fn _decompose_into_primes(input : u64) -> Vec<u64> {

    if input == 1 {
        return Vec::new();
    }

    static mut SEEN_BEFORE : Lazy<HashMap<u64, Vec<u64>>> = Lazy::new(|| { HashMap::new() });

    unsafe {
        if let Some(value) = SEEN_BEFORE.get(&input) {
            return value.clone();
        }
    }

    let mut decomposed_primes = Vec::new();
    let prime_list = Primes::new();
    for prime in prime_list {
        if input % prime == 0 {
            decomposed_primes = _decompose_into_primes(input / prime);
            decomposed_primes.push(prime);
            break;
        }
    }

    unsafe {
        SEEN_BEFORE.insert(input, decomposed_primes.clone());
    }

    return decomposed_primes;
}
