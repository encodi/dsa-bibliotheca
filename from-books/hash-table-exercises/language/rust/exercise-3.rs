use std::collections::HashSet;

fn generate_alphabet() -> HashSet<char> {
    let mut alphabet = HashSet::new();
    for i in b'a'..=b'z' {
        alphabet.insert(i as char);
    }
    alphabet
}

fn missing_letter(sentence: &str) -> Option<char> {
    let alphabet_set = generate_alphabet();
    let mut sentence_set = HashSet::new();

    for letter in sentence.to_lowercase().chars() {
        if letter.is_alphabetic() {
            sentence_set.insert(letter);
        }
    }

    alphabet_set.difference(&sentence_set).cloned().next()
}

// Write a function that acceps a string that contains all the letters of the alphabet
// except one and returns the missing letter. For example, the string "the quick brown box
// jumps over a lazy dog" contains all the letters of the alphabet except the letter, "f".
// The function should have a time complexity of O(n).

fn main() {
    match missing_letter("the quick brown box jumps over a lazy dog") {
        Some(missing) => println!("Missing letter: {}", missing),
        None => println!("No letters missing"),
    }
}
