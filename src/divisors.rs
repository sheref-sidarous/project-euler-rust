
use crate::primes::Primes;
use once_cell::sync::Lazy;
use std::collections::HashMap;

// loop
pub fn _decompose_into_primes(input : u64) -> Vec<u64> {
    
    let mut reminder = input;
    let mut decomposed_primes = Vec::new();
    while reminder != 1 {
        let prime_list = Primes::new();
        for prime in prime_list {
            if reminder % prime == 0 {
                reminder = reminder / prime;
                decomposed_primes.push(prime);
                break;
            }
        }
    }

    return decomposed_primes;
}

// recursive 
pub fn decompose_into_primes(input : u64) -> Vec<u64> {

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
            decomposed_primes = decompose_into_primes(input / prime);                
            decomposed_primes.push(prime);
            break;
        }
    }

    unsafe {
        SEEN_BEFORE.insert(input, decomposed_primes.clone());
    }

    return decomposed_primes;
}
