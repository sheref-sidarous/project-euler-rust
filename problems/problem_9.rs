
struct Primes {
    discovered : Vec<u64>
}

impl Iterator for Primes {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let prime = if self.discovered.len() == 0 {
            2
        } else {
            let mut inspected : u64 = *self.discovered.last().unwrap();
            'outer : loop {
                inspected += 1;
                for i in self.discovered.iter() {
                    if inspected % i == 0 {
                        // the inspected number is devisable by i, therfore not a prime
                        continue 'outer;
                    }
                }

                // if you are here, then inspected is a prime
                break inspected;
            }
        };
        self.discovered.push(prime);
        return Option::Some(prime);
    }
}

fn main() {
    let mut primes = Primes {
        discovered : Vec::new(),
    };

    let mut sum = 0;

    loop {
        let next_prime = primes.next().unwrap();
        if next_prime > 1000u64 {
            break;
        }
        sum += next_prime;
    }

    println!("{}", sum);
    
}
