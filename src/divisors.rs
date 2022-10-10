
use crate::primes::Primes;


pub fn decompose_into_primes(input : u64) -> Vec<u64> {
    
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
