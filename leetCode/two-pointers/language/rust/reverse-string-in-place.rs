fn main() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    reverseString(&mut s);
    println!("{:?}", s);
}

fn reverseString(s: &mut Vec<char>) {
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        s.swap(i, j);
        i += 1;
        j -= 1;
    }
}
