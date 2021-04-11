pub fn main() {
    song_decoder("WUBAWUBWUBC");
}

// Purge all "WUB"s between words, leaving whitespaces
fn song_decoder1(song: &str) -> String {
    // "WUBI"
    let song = song.to_string();
    // let res = song.remove("WUB");
    // println!("{:?}", res);

    let mut out: Vec<String> = Vec::new();
    let mut buffer = String::new();
    // buffer.
    for (i, char) in song.chars().enumerate() {
        buffer.push_str(&char.to_string());
        if buffer.contains("WUB") {
            buffer.pop();
            buffer.pop();
            buffer.pop();
            if buffer.len() != 0 {
                out.push(buffer.clone());
            }
            buffer.clear();
        }
    }
    if buffer.len() != 0 {
        out.push(buffer);
    }
    out.join(" ")
}

// best
fn song_decoder(song: &str) -> String {
    song.to_string()
        .replace("WUB", " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
        .to_string()
}

// personal best
// fn song_decoder(song: &str) -> String {
//     let re = Regex::new(r"(WUB)*");
//     // song.to_string().replace()
// }

#[cfg(test)]
mod tests {
    use super::song_decoder;

    #[test]
    fn returns_expected() {
        assert_eq!(song_decoder("WUBAWUBWUBC"), "A C");
        assert_eq!(song_decoder("AWUBWUBWUBBWUBWUBWUBC"), "A B C");
        assert_eq!(song_decoder("WUBAWUBBWUBCWUB"), "A B C");
        assert_eq!(song_decoder("AWUBBWUBC"), "A B C");
    }
}
