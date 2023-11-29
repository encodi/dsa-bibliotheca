use std::collections::HashMap;

fn first_non_duplicate(sentence: &str) -> Option<char> {
    let mut char_count = HashMap::new();

    for letter in sentence.chars() {
        *char_count.entry(letter).or_insert(0) += 1;
    }

    for letter in sentence.chars() {
        if let Some(&count) = char_count.get(&letter) {
            if count == 1 {
                return Some(letter);
            }
        }
    }

    None
}

// Write a function that returns the first non-duplicated character in a string.
// For example, the string "minimum" has two characters that only exist once, the "n" and "u",
// so your function should return the "n", since it occurs first. The function should have an efficiency of O(n).

fn main() {
    match first_non_duplicate("minimum") {
        Some(letter) => println!("First non-duplicate character: {}", letter),
        None => println!("No non-duplicate characters found"),
    }
}
