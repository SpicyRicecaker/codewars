use std::char;
pub fn main() {
    let res = nb_dig(10, 1);
    println!("{}", res);
}

// old
fn nb_dig1(n: i32, d: i32) -> i32 {
    // Find squares less than n
    let dchar = char::from_digit(d as u32, 10).unwrap();
    let nsquares = (0..n + 1).map(|num| num.pow(2));
    // Then Find digits in n
    let mut big_count = 0;
    for square in nsquares {
        let mut count = 0;
        for char in square.to_string().chars() {
            if dchar == char {
                count += 1;
            }
        }
        big_count += count;
    }
    big_count
}

// new
fn nb_dig2(n: i32, d: i32) -> i32 {
    let dchar = char::from_digit(d as u32, 10).unwrap();
    (0..n + 1)
        .map(|num| num.pow(2).to_string())
        .collect::<Vec<String>>()
        .iter()
        .flat_map(|string| {
            string
                .chars()
                .map(|char| return if char == dchar { 1 } else { 0 })
        })
        .sum()
}

// Reduce is basically fold, without an initial value
// Reduce makes assumptions about the type of accumulator, fold doesn't
//
// Matches actually returns an iterator of pattern matches, so you can use that and .count() an interator 
//
// Flat map removes nested operators which is really useful,
//
// You can have inline ternary operators in rust by using `return if {} else {}`
//
// Finally, you can have range end at the number by doing `(0..=n)` instead of `(0..n+1)`

fn nb_dig(n: i32, d: i32) -> i32 {
    let dchar = d.to_string();
    (0..=n).map(|n| n.pow(2).to_string()).fold(0, |acc, x| acc + x.matches(&dchar).count() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i32, d: i32, exp: i32) -> () {
        println!("n: {:?}", n);
        println!("d: {:?}", d);
        let ans = nb_dig(n, d);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(550, 5, 213);
        dotest(5750, 0, 4700);
    }
}
