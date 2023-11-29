use std::collections::HashMap;

fn main() {
    let vec1 = vec![1, 2, 2, 1];
    let vec2 = vec![2, 2];
    let inter = intersection(vec1, vec2);
    println!("{:?}", inter);
}

// Write a function that returns the intersection of two arrays. The intersection is a third array
// that contains all values contained within the first two arrays. For example, the intersection
// of [1, 2, 3] and [2, 3, 4] is [2, 3]. Your function should have a complexity of O(n). Do not use
// any built-in functions in the language used.

fn intersection(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut result = Vec::new();

    for &letter in &arr1 {
        *map.entry(letter).or_insert(0) += 1;
    }

    for &letter in &arr2 {
        if let Some(count) = map.get_mut(&letter) {
            if *count > 0 {
                result.push(letter);
                *count -= 1;
            }
        }
    }

    result
}
