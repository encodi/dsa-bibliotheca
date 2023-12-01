// Write a function that accepts an array of strings and returns the total number of characters across all the strings.
// For example, if the input array is ["ab", "c", "def", "ghij"], the output should be 10 since there are 10 characters
// in total.
fn number_of_chars(arr: &[&str]) -> usize {
    match arr.split_first() {
        Some((first, rest)) => first.len + number_of_chars(rest),
        None => 0,
    }
}

fn main() {
    let strings = vec!["ab", "c", "def", "ghij"];
    println!("{}", number_of_chars(&strings));
}
