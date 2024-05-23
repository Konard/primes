use primes::{Sieve, PrimeSet};

fn main() {
    let limit = 100; // You can change this limit to any number you want
    let mut pset = Sieve::new();
    let primes: Vec<u64> = pset.iter().take_while(|&p| p <= limit).collect();

    for &prime in &primes {
        println!("(p_n - 1) / 2 for prime {} is {}", prime, (prime - 1) / 2);
    }
}