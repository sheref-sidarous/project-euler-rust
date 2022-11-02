
use memoization_macro::memoize;

#[memoize]
fn collatz_sequence_steps(n : u64) -> u64 {

    if n == 1 {
        return 1;
    }

    let next = if n % 2 == 0 {
        // even number
        n / 2
    } else {
        // odd number
        3 * n + 1
    };

    1 + collatz_sequence_steps(next)
}

pub fn solution() {
    let mut max_n = 1;
    let mut max_steps = 1;
    for i in 1..1000001 {
        let steps = collatz_sequence_steps(i);
        if steps > max_steps {
            max_steps = steps;
            max_n = i;
        }
    }

    println!("n={} results in {} steps", max_n, max_steps);
}