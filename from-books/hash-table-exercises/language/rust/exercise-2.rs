use std::collections::HashMap;

fn main() {
    let arr = vec![
        "a".to_string(),
        "b".to_string(),
        "a".to_string(),
        "c".to_string(),
    ];
    match first_duplicate(&arr) {
        Some(duplicate) => println!("First duplicate: {}", duplicate),
        None => println!("No duplicates found"),
    }
}

// Write a funciton that accepts an array of strings and returns the first duplicate value it finds.
// For example, if the array is ["a", "b", "c", "d", "c", "e", "f"], the function should return "c",
// since it's duplicated within the array. (You can assume that there's one pair of duplicates within the array.).
// Make sure the function has an efficiency of O(n).

fn first_duplicate(arr: &[String]) -> Option<String> {
    let mut hash_map = HashMap::new();

    for letter in arr {
        let count = hash_map.entry(letter.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            return Some(letter.clone());
        }
    }

    None
}
