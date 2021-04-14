pub fn main() {
    who_is_next(
        &vec![
            Name::Sheldon,
            Name::Leonard,
            Name::Penny,
            Name::Rajesh,
            Name::Howard,
        ],
        2,
    );
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Name {
    Sheldon,
    Leonard,
    Penny,
    Rajesh,
    Howard,
}

/// Just to make code look a bit nicer
type Names = Vec<Name>;

/// Will return the `Name` of the person who will drink the `n`-th cola.
fn who_is_next_1(names: &Names, n: usize) -> Name {
    let mut names = vec![Name::Sheldon, Name::Leonard];

    // Print up until the 10th cola
    for _ in 1..n {
        let drinker = names.remove(0);
        names.push(drinker);
        names.push(drinker);
    }
    return *names.get(0).unwrap();

    // Come up with geometric series sum formula
    // Retrieve n from that formula
    // let i = (((n + 1) as f64).log2() + 0.5_f64).floor();
    // println!("original log={}", (((n + 1) as f64).log2()));
    // println!("i={} becomes {}", i, (i % 5_f64) as usize);
    // // Names.get(Find remainder of n % 5)
    // *names.get((i % 5_f64) as usize - 1).unwrap()
}

fn if_arithmetic_sequence_lol_i_smart(names: &Names, n: usize) -> Name {
    // (5/2) * x(x+1)/2 = s
    let x = ((1_f64 + (8_f64 / 5_f64) * n as f64).sqrt() - 1_f64) / 2_f64;
    // find greatest integer
    let x_n_1 = x.trunc();
    // find sum at greatest integer
    let x_n_1_sum = ((5_f64 / 2_f64) * x_n_1 * (x_n_1 + 1_f64)) as usize;
    // find diff between the two
    let diff = n - x_n_1_sum;
    // divide by num of names
    let index = (diff as f64 / names.len() as f64).trunc() as usize;
    println!(
        "for sum={}, x={}, x_n_1={}, x_n_1_sum={}, diff={}, index={}",
        n, x, x_n_1, x_n_1_sum, diff, index
    );

    *names.get(index).unwrap()
}

fn who_is_next(names: &Names, n: usize) -> Name {
    let len = names.len();
    // Solving geometric series equation`n-1=5(2^x-1)` for x, rounding down to get the *past* term`n-1`
    let x = ((n as f64 - 1_f64) / len as f64 + 1_f64).log2().trunc();
    // find # of duplicated persons at *current* term `n`
    let current_duplications = 2_usize.pow(x as u32);
    // find total number of colas drunk at `n-1` using geometric series formula
    let x_sum = len * (2_usize.pow(x as u32) - 1);
    // find diff between colas drunk at `n-1` and colas drunk at n
    let diff = n - x_sum - 1;
    // divide diff by # of current duplicated ppl
    let index = (diff as f64 / current_duplications as f64).trunc() as usize;
    *names.get(index).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let names = &vec![
            Name::Sheldon,
            Name::Leonard,
            Name::Penny,
            Name::Rajesh,
            Name::Howard,
        ];

        assert_eq!(who_is_next(names, 1), Name::Sheldon);
        assert_eq!(who_is_next(names, 6), Name::Sheldon);
        assert_eq!(who_is_next(names, 52), Name::Penny);
        assert_eq!(who_is_next(names, 7_230_702_951), Name::Leonard);
    }
    #[test]
    fn howard_tests() {
        let names = &vec![
            Name::Sheldon,
            Name::Leonard,
            Name::Penny,
            Name::Rajesh,
            Name::Howard,
        ];

        assert_eq!(who_is_next(names, 5), Name::Howard);
        assert_eq!(who_is_next(names, 15), Name::Howard);
        assert_eq!(who_is_next(names, 35), Name::Howard);
        assert_eq!(who_is_next(names, 155), Name::Howard);
    }

    #[test]
    fn leonard_tests() {
        let names = &vec![
            Name::Sheldon,
            Name::Leonard,
            Name::Penny,
            Name::Rajesh,
            Name::Howard,
        ];

        assert_eq!(who_is_next(names, 8), Name::Leonard);
        assert_eq!(who_is_next(names, 11), Name::Penny);
        assert_eq!(who_is_next(names, 13), Name::Rajesh);
        assert_eq!(who_is_next(names, 155), Name::Howard);
    }
}
