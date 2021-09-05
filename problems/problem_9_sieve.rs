


fn main() {
    const SIZE :usize = 2000000;
    let mut is_prime : [bool; SIZE] = [true; SIZE];
    
    let mut i = 2;
    let mut sum = 0;
    while i < is_prime.len() {
        if is_prime[i] == true {
            // this means that i is a prime
            sum += i;

            // mark all its multiples as non-primes
            let mut index = 2;
            while i * index < is_prime.len() {
                is_prime[i * index] = false;
                index += 1;
            }
        }
        i += 1;
    }

    println!("Sum is {}", sum);



}