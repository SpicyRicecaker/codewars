use std::time::{Instant};
fn main() {
    testing(2, 100, 110, Some((101, 103)));
    testing(4, 100, 110, Some((103, 107)));
    testing(8, 30000, 100000, Some((30089, 30097)));
    testing(11, 30000, 100000, None);
    testing(2, 10000000, 11000000, Some((10000139, 10000141)));
}

// g gap, m starting, n ending
fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let snap = Instant::now();
    // Convert g to unsigned
    let g = g.abs() as u64;
    // Init a new list of primes
    let mut primes: Vec<u64> = Vec::new();

    // For every number leading up to and including n
    for i in m..n + 1 {
        // Check if it is in the right form
        if is_prime(i) {
            // Then add it
            primes.push(i);
        }
    }
    println!("Finding primes took {:?}", snap.elapsed());

    let snap = Instant::now();
    // Naively compare diffs
    let length = primes.len();
    for (i, &num) in primes.iter().enumerate() {
        for j in i..length {
            if g == primes[j] - num {
                println!("Finding res took {:?}", snap.elapsed());
                return Some((num, primes[j]));
            }
        }
    }
    println!("Finding res took {:?}", snap.elapsed());
    None
}

// Takes in number and returns a prime based on the fact that
// All primes over 2,3 are in the form 6k + i where i = -1, 0, 1, 2, 3, 4 and k is a positive integer
// Algo from Wikipedia, [Primality Test](https://en.wikipedia.org/wiki/Primality_test)

pub fn is_prime(n: u64) -> bool {
    // If the number is less than or equal to three,
    // Return prime basically if it is 2 or 3
    if n <= 3 {
        return n > 1;
    }
    // Then, if we check if the number is divisible by 2 or 3,
    // We don't have to check if i=2, 3, or 4, and thus,
    // Any prime will then be in the form 6k +- i
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    // Next, we just have to check if the number divided by the numbers in the form 6k +- 1 leading up to the \sqrt{number}
    // are divisble, and if all of them are not, it's prime!
    let root = (n as f64).sqrt().floor() as u64;
    for i in (5..root + 1).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }
    true
}

fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    let res = step(g, m, n);
    println!("{:?}, {:?}", res, exp);
    assert_eq!(res, exp);
}

#[test]
fn tests() {
    testing(2, 100, 110, Some((101, 103)));
    testing(4, 100, 110, Some((103, 107)));
    testing(8, 30000, 100000, Some((30089, 30097)));
    testing(11, 30000, 100000, None);
    testing(2, 10000000, 11000000, Some((10000139, 10000141)));
}