fn binarySearch(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return mid as i32;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    -1
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target = 5;
    let index = binarySearch(&arr, target);
    println!("index: {}", index);
}
