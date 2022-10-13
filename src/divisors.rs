
use crate::primes::Primes;
use once_cell::sync::Lazy;
use std::collections::HashMap;


type Power = u64;
type Prime = u64;

pub fn _prime_facorization(input : u64) -> HashMap<Prime, Power> {
    
    let mut reminder = input;
    let mut facorized_primes = HashMap::new();
    
    let prime_list = Primes::new();
    for prime in prime_list {
        let mut power = 0;
        while reminder % prime == 0 {
            reminder = reminder / prime;
            power += 1;
        }
        if power > 0 {
            facorized_primes.insert(prime, power);
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
        | acc, factor | {  acc * (factor.1 + 1)} );

    return no_of_divisors;
}

// recursive 
pub fn prime_facorization(input : u64) -> HashMap<Prime, Power> {

    if input == 1 {
        return HashMap::new();
    }

    static mut SEEN_BEFORE : Lazy<
                                    HashMap<u64, // function input
                                        HashMap<Prime, Power> // function output
                                    >
                             > = Lazy::new(|| { HashMap::new() });

    unsafe {
        if let Some(value) = SEEN_BEFORE.get(&input) {
            return value.clone();
        }
    }

    let prime_list = Primes::new();
    let mut decomposed_primes = HashMap::new();
    for prime in prime_list {
        if input % prime == 0 {
            decomposed_primes = prime_facorization(input / prime);
            let new_power = if let Some(value) = decomposed_primes.get(&prime) {
                *value + 1
            } else {
                1
            };
            decomposed_primes.insert(prime, new_power);
            
            break;
        }
    }

    unsafe {
        SEEN_BEFORE.insert(input, decomposed_primes.clone());
    }

    return decomposed_primes;
}
