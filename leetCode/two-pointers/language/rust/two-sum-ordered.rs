fn twoSumOrdered(arr: &[i32], target: i32) -> (i32, i32) {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left < right {
        let sum = arr[left] + arr[right];
        if sum == target {
            return (left as i32, right as i32);
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    (-1, -1)
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target = 5;
    let (index1, index2) = twoSumOrdered(&arr, target);
    println!("index1: {}, index2: {}", index1, index2);
}
