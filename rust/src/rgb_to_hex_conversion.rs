pub fn main() {
    rgb(255, 254, 200);
}

// enum HexDigit {
//     Letter(char),
//     Number(i32)
// }

fn rgb(r: i32, g: i32, b: i32) -> String {
    let mut res = String::new();
    let to_hex_digit = |int: i32| match int {
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        14 => 'E',
        15 => 'F',
        _ => int.to_string().chars().into_iter().next().unwrap(),
    };

    for n in [r, g, b].iter() {
        res.push(to_hex_digit(((*n as f64) / 16_f64).trunc() as i32));
        res.push(to_hex_digit(n % 16));
    }

    res
}

fn append_char() {
    let mut c = String::new();
    let char_a = 'A';
    let char_b = 'B';
    c.push(char_a);
    c.push(char_b);
    println!("{}", c);
}

macro_rules! compare {
    ( $got : expr, $expected : expr ) => {
        if $got != $expected {
            panic!("Got: {}\nExpected: {}\n", $got, $expected);
        }
    };
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}
