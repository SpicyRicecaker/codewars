pub fn main () {
    println!("{}", alphabet_position("The sunset sets at twelve o' clock."));
}

fn alphabet_position(text: &str) -> String {
    let alphabet = ['a','b','c','d','e','f','g','h','i','j','k','l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y','z'];
    let mut out: String = String::new();

    for char in text.chars() {
        if char.is_alphabetic() {
            let position = alphabet.iter().position(|&test| test == char.to_ascii_lowercase()).unwrap();
            out.push_str(&format!(" {}", &position.to_string()));
        }
    }
    String::from(out.trim())
}