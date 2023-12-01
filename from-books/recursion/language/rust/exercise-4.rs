fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut result: Vec<i32> = vec![];

    get_even_numbers(numbers, &mut result);
    println!("{:?}", result);
}

fn get_even_numbers(numbers: Vec<i32>, result: &mut Vec<i32>) {
    if numbers.len() == 0 {
        return;
    }
    if numbers[0] % 2 == 0 {
        result.push(numbers[0])
    }

    return get_even_numbers(numbers[1..].to_vec(), result);
}.to_vec()
