fn main() {
    println!("{}", findMaxAverage(vec![1, 12, -5, -6, 50, 3], 4));
}

fn findMaxAverage(nums: Vec<i32>, k: i32) -> f64 {
    let mut sum = 0;
    for i in 0..k as usize {
        sum += nums[i];
    }
    let mut max = sum;
    for i in k as usize..nums.len() {
        sum += nums[i] - nums[i - k as usize];
        max = max.max(sum);
    }
    max as f64 / k as f64
}
