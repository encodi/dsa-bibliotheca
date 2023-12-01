// There is a numerical sequence called "triangular numbers", the pattern begins 1,3,6,10,15,21 and
// continues with the nth number in the pattern, which is N + previous number. For example,
// the 7th number in the sequence is 28, since it's 7 (which is N) plus 21 the previous number in the sequence
// Write a function that accepts a number N and returns the correct number from the series. For example, for
// 7 it will return 28.

fn main() {
    let number: i32 = 7;
    println!("{}", get_triangular_numbers(number));
}

fn get_triangular_numbers(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    return n + get_triangular_numbers(n - 1);
}
