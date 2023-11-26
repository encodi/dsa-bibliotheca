fn bubbleSort(arr: &mut [i32]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }
    }
}

fn main() {
    let mut arr = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    bubbleSort(&mut arr);
    println!("{:?}", arr);
}
