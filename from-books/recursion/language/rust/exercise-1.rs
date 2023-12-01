fn main() {
    println!("{}", sum(1, 10));
}

fn sum(low: i32, high: i32) -> i32 {
    if low > high {
        0
    } else {
        high + sum(low, high - 1)
    }
}
