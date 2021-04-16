use std::collections::{HashMap, HashSet};

pub fn main() {}

fn prime_factors(l: i64) -> HashSet<i64> {
    let mut factors: HashSet<i64> = HashSet::new();
    let mut l = l;

    while l != 1 {
        for i in 2..=l {
            if l % i == 0 {
                factors.insert(i);
                // out.push(i);
                l /= i;
                break;
            }
        }
    }
    factors
}

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let mut table: HashMap<i64, i64> = HashMap::new();

    for num in l {
        for factor in prime_factors(num) {
            let entry = table.entry(factor).or_insert(0);
            *entry += num;
        }
    }
    
    // println!("{:?}", table);
    
    let mut res = table.into_iter().collect::<Vec<(i64, i64)>>();
    res.sort_unstable();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) {
        assert_eq!(sum_of_divided(l), exp)
    }

    fn testing_factors(l: i64, exp: HashSet<i64>) {
        assert_eq!(prime_factors(l), exp)
    }

    #[test]
    fn basics_sum_of_divided() {
        testing(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)]);
        testing(
            vec![15, 21, 24, 30, 45],
            vec![(2, 54), (3, 135), (5, 90), (7, 21)],
        );
    }

    #[test]
    fn test_prime_factors() {
        let gen = |vec: Vec<i64>| vec.into_iter().collect::<HashSet<i64>>();
        testing_factors(12, gen(vec![2, 3]));
        // testing_factors(12, [2,3]);
        testing_factors(15, gen(vec![5, 3]));
        testing_factors(1, gen(vec![]));
    }
}
