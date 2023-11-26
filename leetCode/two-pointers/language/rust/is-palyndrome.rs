fn isPalyndrome(s: &str) -> bool {
    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        let leftChar = s.chars().nth(left).unwrap();
        let rightChar = s.chars().nth(right).unwrap();
        if leftChar != rightChar {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

fn main() {
    let s = "abba";
    let result = isPalyndrome(s);
    println!("result: {}", result);
}
