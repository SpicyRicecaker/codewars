use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub fn main() {
    let res = sum_pairs(&[5, 9, 13, -3], 10);
    dbg!(res);
}

fn sum_pairs_old(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let start = Instant::now();
    let mut beg_index = 0;
    let mut end_index = ints.len() - 1;

    let mut pair = None;

    // double loop, if 2 sums = sum && end index < index
    // then store pair and its indices
    // cache.get(*min, *max)
    for (i, min) in ints.iter().enumerate() {
        for (j, max) in ints[i + 1..].iter().enumerate() {
            let j = j + i + 1;
            if min + max == s && j <= end_index && i >= beg_index {
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
//     let candidates: HashMap<i8, bool> = HashMap::new();
//     ints.iter().map(|n| match candidates.entry(s-n) {
//         std::collections::hash_map::Entry::Occupied(_) => {
//             return Some((s-n, *n));
//         }
//         std::collections::hash_map::Entry::Vacant(_) => {
//             return
//         }
//     });

//     None
// }

// Fkin big brain right here,
// We use a hashmap to stored all our seen sets
// Then we can lookup seen values as we go along
// Our current number is the rightmost, so any match would be the best one!
fn sum_pairs_personal(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut candidates: HashMap<i8, bool> = HashMap::new();
    for n in ints.iter() {
        // Check if there is a match in the hashmap
        if let std::collections::hash_map::Entry::Occupied(_) = candidates.entry(s - n) {
            // If there is a match return it
            return Some((s - n, *n));
        }
        // Otherwise, add our current number to possible pairings
        candidates.entry(*n).or_insert(true);
    }

    None
}

// Best solution, uses a hashset, which is basically hashmap with t/f, so it's slightly better. Same algo tho
fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut seen: HashSet<i8> = HashSet::new();
    for &n in ints.iter() {
        if seen.contains(&(s - n)) {
            return Some((s - n, n));
        }
        seen.insert(n);
    }
    None
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
    // let l9 = [1; 10000];
    assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    assert_eq!(sum_pairs(&l3, -7), None);
    assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
    assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
    assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
    assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
    // assert_eq!(sum_pairs(&l9, 127), Some((13, -3)))
}
