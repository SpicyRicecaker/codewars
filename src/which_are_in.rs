use itertools::Itertools;
// Find strings that are actual substrings, then sort lexicographically (basically alphabetically, a, ab, abc, bc)

// Naive solution
fn in_array_d(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for strng in arr_a {
        if arr_b.iter().any(|v| v.contains(strng)) {
            out.push(strng.to_string());
        }
    }
    out.sort();
    out.dedup();
    out
}

// Best sol using native library
fn in_array_df(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut contains = arr_a
        .iter()
        .filter(|sub| arr_b.iter().any(|word| word.contains(*sub)))
        .map(|word| word.to_string())
        .collect::<Vec<String>>();
    contains.sort();
    contains.dedup();

    contains
}

// Best sol. using `itertools`
fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    arr_a
        .iter()
        .filter(|sub| arr_b.iter().any(|word| word.contains(*sub)))
        .map(|word| word.to_string())
        // .collect::<Vec<String>>()
        .unique()
        .sorted()
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            in_array(
                &["xyz", "live", "strong"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["live", "strong"]
        );

        assert_eq!(
            in_array(
                &["live", "strong", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );

        assert_eq!(
            in_array(
                &["tarp", "mice", "bull"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            [] as [&str; 0]
        );

        assert_eq!(
            in_array(
                &["live", "strong", "arp", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );
    }
}
