pub fn main() {}

fn make_upper_case(s: &str) -> String {
    s.chars()
        .into_iter()
        .map(|c| c.to_ascii_uppercase())
        .collect::<String>()
}

// best solution
fn best_make_upper_case(s: &str) -> String {
    s.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_upper_case() {
        assert_eq!(make_upper_case("hello"), "HELLO");
    }
    #[test]
    fn test_best_make_upper_case() {
        assert_eq!(best_make_upper_case("hello"), "HELLO");
    }
}
