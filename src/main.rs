
// mod problem_8;
mod problem_12;
//use problem_12::solution;


mod primes;
mod divisors;

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

fn main() {
    
    //println!("{:?}", divisors::prime_facorization(11));
    //println!("{:?}", divisors::number_of_divisors(11));
    problem_12::solution();



}
