use std::time::{Duration, Instant};

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
    // If m comes after 3, include 2 and 3
    if m >= 3 {
        primes.push(2);
        primes.push(3);
    } else {
        // For regular, probably return 2
        return None;
    }

    // For every number leading up to and including n
    for i in 4..n + 1 {
        // Check if it is in the right form
        if is_prime(i) {
            // Then add it
            primes.push(i);
        }
    }
    println!("Primes took {:?}", snap.elapsed());
    // // Print primes
    // for i in primes.iter() {
    //     println!("{}", i);
    // }

    let snap = Instant::now();
    // Get primes larger than m
    let mut m_point = 0;
    for (i, num) in primes.iter().enumerate() {
        if *num > m {
            // println!("{} is greater than {}", *num, m);
            m_point = i;
            break;
        }
    }
    // for (i, &num) in (&primes[m_point..]).iter().enumerate() {
    //     println!("{}", num);
    // }

    let current = &primes[m_point..];
    // // Naively compare diffs
    let length = current.len();
    // println!("length is {}", length);
    // println!("starting at {}", m_point);
    for (i, &num) in current.iter().enumerate() {
        // println!("testing {}", num);
        for j in i..length {
            // println!("{} - {}", &primes[j+m_point], num);
            // println!("j is {}, i is {}, num is {}, prime is {}", j, i, num, current[j]);
            if g == current[j] - num {
                println!("Found {} and {} in {:?}", num, current[j], snap.elapsed());
                return Some((num, current[j]));
            }
        }
    }
    println!("{:?}", snap.elapsed());
    None
}

// Takes in number and returns a prime based on the fact that
// All primes over 2,3 are in the form 6k + i where i = -1, 0, 1, 2, 3, 4 and k is a positive integer
// Algo from Wikipedia, [Primality Test](https://en.wikipedia.org/wiki/Primality_test)
fn is_prime(n: u64) -> bool {
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

// Bad algo
// fn is_prime(n: u64) -> bool {
//     for i in 2..n {
//         if n % i == 0 {
//             return false;
//         }
//     }
//     true
// }

fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(step(g, m, n), exp);
}
