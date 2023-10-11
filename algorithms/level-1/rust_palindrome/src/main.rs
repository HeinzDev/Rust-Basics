fn is_palindrome(phrase: &str) -> bool {
    let phrase_filtered = phrase.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    let len = phrase_filtered.len();

    for i in 0..len / 2 {
        if phrase_filtered.chars().nth(i) != phrase_filtered.chars().nth(len - i - 1) {
            return false;
        }
    }
    true
}

fn main() {
    let word = "tenet";
    let result = is_palindrome(word);

    println!("Is the phrase '{}' a palindrome?: {}", word, result);
}
