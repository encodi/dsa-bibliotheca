fn main() {
    println!(
        "{}",
        is_subsequence("abc".to_string(), "ahbgdc".to_string())
    );
}

fn is_subsequence(s: String, t: String) -> bool {
    let mut s = s.chars().collect::<Vec<_>>();
    let mut t = t.chars().collect::<Vec<_>>();
    let mut i = 0;
    for j in 0..t.len() {
        if i == s.len() {
            return true;
        }
        if s[i] == t[j] {
            i += 1;
        }
    }
    i == s.len()
}
