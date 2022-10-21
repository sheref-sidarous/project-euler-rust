
use once_cell::sync::Lazy;
use core::cell::RefCell;

pub struct Primes<'a> {
    prime_list : &'a RefCell<Vec<u64>>,
    progress : usize,
}

impl Primes<'_> {

    pub fn new<'a>() -> Self {
        static mut SINGLETON_PRIME_LIST : Lazy<RefCell<Vec<u64>>> = Lazy::new(|| {
            let new_list  =  RefCell::new(Vec::new());
            new_list.borrow_mut().push(2);

            return new_list;
        });

        unsafe {
            Primes { prime_list: &SINGLETON_PRIME_LIST, progress: 0 }
        }
    }

    fn discover_next(&self) -> u64 {
        let prime = {
            let mut inspected : u64 = *self.prime_list.borrow().last().unwrap();
            'outer : loop {
                inspected += 1;
                for i in self.prime_list.borrow().iter() {
                    if inspected % i == 0 {
                        // the inspected number is devisable by i, therfore not a prime
                        continue 'outer;
                    }
                }

                // if you are here, then inspected is a prime
                break inspected;
            }
        };
        self.prime_list.borrow_mut().push(prime);
        return prime;
    }
}

impl Iterator for Primes<'_> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {

        let next_prime = 
            if self.progress < self.prime_list.borrow().len() {
                self.prime_list.borrow()[self.progress]
            } else {
                self.discover_next()
            };

        self.progress += 1;
        
        Some(next_prime)
        
    }
}

fn _list_first_few_primes() {
    let my_prime_list = primes::Primes::new();

    let mut i = 0;

    for prime in my_prime_list {
        println!("{}", prime);
        i += 1;
        if i > 10 {
            break;
        }
    }

}

