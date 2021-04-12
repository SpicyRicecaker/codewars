use std::collections::HashMap;
use std::time::{Duration, Instant};

pub fn main() {
    let res = sum_pairs(&[1, 2, 3, 4, 1, 0], 2);
}

struct Cache <'a>{
    sumcache: HashMap<&'a i8, HashMap<&'a i8, i8>>,
}

impl<'a> Cache <'a> {
    fn new() -> Self {
        Cache {
            sumcache: HashMap::new(),
        }
    }

    fn get(&mut self, first_num: &'a i8, second_num: &'a i8) -> i8 {
        let first = self.sumcache.entry(first_num).or_insert_with(HashMap::new);
        let second = first.entry(second_num).or_insert(first_num + second_num);
        println!("successful access of {} + {}", first_num, second_num);

        *second
    }
}

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let start = Instant::now();
    let mut beg_index = 0;
    let mut end_index = ints.len() - 1;

    let mut pair = None;

    // double loop, if 2 sums = sum && end index < index
    // then store pair and its indices
    let mut cache = Cache::new();
    // cache.get(*min, *max)
    for (i, min) in ints.iter().enumerate() {
        for (j, max) in ints[i + 1..].iter().enumerate() {
            let j = j + i + 1;
            if  cache.get(min, max) == s && j <= end_index && i >= beg_index {
                pair = Some((*min, *max));
                beg_index = i;
                end_index = j;
            }
        }
    }

    // let m = ints.iter()
    // .filter(|&&n| n < s)
    // .enumerate()
    // .map(|(i, min)| ints[i+1..].iter().enumerate()
    // .map(|(j,max)|return if min + max == s && j <= end_index && i >= beg_index {Some((*min, *max))} else {None}));
    println!("{} Total Time: {:?}", s, start.elapsed());

    pair
}

// fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
//     let mut beg_index = 0;
//     let mut end_index = ints.len() - 1;

//     let mut pair = None;

//     // double loop, if 2 sums = sum && end index < index
//     // then store pair and its indices
//     ints.iter().enumerate()
//     .map(|(i, min)| ints[i+1..].iter().enumerate()
//     .map(|j,max|return if min + max == s && j <= end_index && i >= beg_index {})

// )

//     pair
// }

#[test]
fn returns_expected() {
    let l1 = [1, 4, 8, 7, 3, 15];
    let l2 = [1, -2, 3, 0, -6, 1];
    let l3 = [20, -13, 40];
    let l4 = [1, 2, 3, 4, 1, 0];
    let l5 = [10, 5, 2, 3, 7, 5];
    let l6 = [4, -2, 3, 3, 4];
    let l7 = [0, 2, 0];
    let l8 = [5, 9, 13, -3];
    let l9 = [1; 10000];
    assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    assert_eq!(sum_pairs(&l3, -7), None);
    assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
    assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
    assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
    assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
    assert_eq!(sum_pairs(&l9, 127), Some((13, -3)))
}
