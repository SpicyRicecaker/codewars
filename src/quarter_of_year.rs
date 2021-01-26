fn main() {
    quarter_of(12);
    quarter_of(11);
    quarter_of(6);
    quarter_of(1);
}

fn quarter_of(month: u8) -> u8 {
    // println!("{}", (month - 1)  / 3);
    ((month - 1) / 3) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(quarter_of(3), 1, "Month 3 = quarter 1");
        assert_eq!(quarter_of(8), 3, "Month 8 = quarter 3");
        assert_eq!(quarter_of(11), 4, "Month 11 = quarter 4");
    }
}
