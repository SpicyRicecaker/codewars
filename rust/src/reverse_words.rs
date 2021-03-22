fn reverse_words_first(str: &str) -> String {
    let mut out = String::new();
    let mut buffer = String::new();
    // For all characters
    for c in str.as_bytes().iter() {
        // If it is a space
        if *c == b' ' {
            // Check if buffer is empty
            if buffer.len() != 0 {
                // Reverse and add word
                out.push_str(&buffer.chars().rev().collect::<String>());
                // Clear buffer
                buffer.clear();
            }
            // Push space to out
            out.push(' ');
        } else {
            // Otherwise add word
            buffer.push(*c as char);
        }
    }
    // Reverse and add buffer
    out + &buffer.chars().rev().collect::<String>()
}

// If we know word is one space (best solution)
fn reverse_words(str: &str) -> String {
    str.to_string()
        .split(" ")
        .map(|word| word.chars().rev().collect())
        .collect::<Vec<String>>()
        .join(" ")
}


// Rust tests
#[test]
fn sample_test() {
    assert_eq!(
        reverse_words("The quick brown fox jumps over the lazy dog."),
        "ehT kciuq nworb xof spmuj revo eht yzal .god"
    );
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(reverse_words("a b c d"), "a b c d");
    assert_eq!(
        reverse_words("double  spaced  words"),
        "elbuod  decaps  sdrow"
    );
}
