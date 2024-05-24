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

// EXPERIMENT 3

// use primes::{Sieve, PrimeSet, is_prime};

// fn main() {
//     let limit = 1000; // You can change this limit to any number you want
//     let mut pset = Sieve::new();
//     let primes: Vec<u64> = pset.iter().take_while(|&p| p <= limit).collect();

//     for pair in primes.windows(2) {
//         if let [prev_prime, current_prime] = pair {
//             let difference = current_prime - prev_prime - 1;
//             let is_difference_prime = is_prime(difference) || difference == 1;

//             println!(
//                 "Difference (p[n] - p[n-1] - 1) for primes {} and {} is {}. Is it prime? {}",
//                 current_prime,
//                 prev_prime,
//                 difference,
//                 is_difference_prime
//             );
//         }
//     }
// }

// EXPERIMENT 4

// use primes::{PrimeSet, Sieve};

// // Function to find all combinations of primes that sum to a target number
// fn find_prime_sums(primes: &Vec<u64>, target: u64) -> Vec<Vec<u64>> {
//     let mut result = Vec::new();
//     let mut current_combination = Vec::new();
//     helper(primes, target, 0, &mut current_combination, &mut result);
//     result
// }

// fn helper(
//     primes: &Vec<u64>,
//     target: u64,
//     start_index: usize,
//     current_combination: &mut Vec<u64>,
//     result: &mut Vec<Vec<u64>>
// ) {
//     if target == 0 {
//         result.push(current_combination.clone());
//         return;
//     }

//     for i in start_index..primes.len() {
//         if primes[i] > target {
//             return;
//         }
//         current_combination.push(primes[i]);
//         helper(primes, target - primes[i], i, current_combination, result);  // Ensure no overflow here
//         current_combination.pop();
//     }
// }

// fn main() {
//     // Set a limit up to which we want to generate prime numbers
//     let limit = 50;
//     let mut s = Sieve::new();
//     let primes: Vec<u64> = s.iter().take_while(|p| *p <= limit).collect();

//     // Print each prime and its representation as a sum of the smallest primes
//     for &prime in &primes {
//         let combinations = find_prime_sums(&primes, prime);
//         println!("Prime: {}\nCombinations:", prime);
//         for combination in combinations {
//             println!("{:?} = {}", combination, combination.iter().sum::<u64>());
//         }
//     }
// }

// EXPERIMENT 5

// use primes::{PrimeSet, Sieve};

// // Function to find all combinations of primes that sum to a target number
// fn find_prime_sums(primes: &Vec<u64>, target: u64) -> Vec<Vec<u64>> {
//     let mut result = Vec::new();
//     let mut current_combination = Vec::new();
//     helper(primes, target, 0, &mut current_combination, &mut result);
//     result
// }

// fn helper(
//     primes: &Vec<u64>,
//     target: u64,
//     start_index: usize,
//     current_combination: &mut Vec<u64>,
//     result: &mut Vec<Vec<u64>>
// ) {
//     if target == 0 {
//         result.push(current_combination.clone());
//         return;
//     }

//     for i in start_index..primes.len() {
//         if primes[i] > target {
//             return;
//         }
//         current_combination.push(primes[i]);
//         helper(primes, target - primes[i], i + 1, current_combination, result);  // Move to the next prime
//         current_combination.pop();
//     }
// }

// fn main() {
//     // Set a limit up to which we want to generate prime numbers
//     let limit = 50;
//     let mut s = Sieve::new();
//     let primes: Vec<u64> = s.iter().take_while(|p| *p <= limit).collect();

//     // Print each prime and its representation as a sum of the smallest primes
//     for &prime in &primes {
//         let combinations = find_prime_sums(&primes, prime);
//         println!("Prime: {}\nCombinations:", prime);
//         for combination in combinations {
//             println!("{:?} = {}", combination, combination.iter().sum::<u64>());
//         }
//     }
// }

// EXPERIMENT 6

use primes::{Sieve, PrimeSet};

fn main() {
    let mut pset = Sieve::new(); // Initialize PrimeSet to generate primes
    let mut primes = vec![1]; // Include 1 as the first "prime"
    let mut iter = pset.iter();

    while let Some(prime) = iter.next() {
        primes.push(prime);
        if !can_generate_prime(&primes, prime) {
            panic!("Cannot generate prime {} using the specified rule", prime);
        }
        // if prime > 120 {
        //     return;
        // }
    }
}

fn can_generate_prime(primes: &[u64], prime: u64) -> bool {
    let i = primes.len() - 2;
    let previous_prime = primes[i];
    let difference = prime - previous_prime;
    
    for j in (0..i+1).rev() {
        if primes[j] > difference {
            continue;
        }
        let sum = previous_prime + primes[j];
        if sum == prime {
            if primes[j] == 2 {
                println!("{} = {} + 1 + 1", prime, previous_prime);
            } else {
                println!("{} = {} + {}", prime, previous_prime, primes[j]);
            }
            if primes[j] > 2 {
                panic!("This is greater than 2.");
            }
            return true;
        }
        for k in (0..j+1).rev() {
            if sum + primes[k] == prime {
                if primes[k] > 13 {
                    panic!("Last element of the sum is {}.", primes[k]);
                }
                println!("{} = {} + {} + {}", prime, previous_prime, primes[j], primes[k]);
                if primes[j] == 2 && primes[k] == 1 {
                    panic!("2 + 1");
                }
                return true;
            }
        }
    }
    false
}

