// EXPERIMENT 1

// use primes::{Sieve, PrimeSet};

// fn main() {
//     let limit = 100; // You can change this limit to any number you want
//     let mut pset = Sieve::new();
//     let primes: Vec<u64> = pset.iter().take_while(|&p| p <= limit).collect();

//     for &prime in &primes {
//         println!("(p_n - 1) / 2 for prime {} is {}", prime, (prime - 1) / 2);
//     }
// }

// EXPERIMENT 2

// use primes::{Sieve, PrimeSet};

// fn main() {
//     let limit = 1000; // You can change this limit to any number you want
//     let mut pset = Sieve::new();
//     let primes: Vec<u64> = pset.iter().take_while(|&p| p <= limit).collect();

//     for pair in primes.windows(2) {
//         if let [prev_prime, current_prime] = pair {
//             println!("p[n] - p[n-1] - 1 for primes {} and {} is {}", current_prime, prev_prime, current_prime - prev_prime - 1);
//         }
//     }
// }

use primes::{Sieve, PrimeSet, is_prime};

fn main() {
    let limit = 1000; // You can change this limit to any number you want
    let mut pset = Sieve::new();
    let primes: Vec<u64> = pset.iter().take_while(|&p| p <= limit).collect();

    for pair in primes.windows(2) {
        if let [prev_prime, current_prime] = pair {
            let difference = current_prime - prev_prime - 1;
            let is_difference_prime = is_prime(difference) || difference == 1;

            println!(
                "Difference (p[n] - p[n-1] - 1) for primes {} and {} is {}. Is it prime? {}",
                current_prime,
                prev_prime,
                difference,
                is_difference_prime
            );
        }
    }
}