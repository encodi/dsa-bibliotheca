fn insertionSort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn main() {
    let mut arr = [1, 5, 2, 4, 3];
    insertionSort(&mut arr);
    println!("{:?}", arr);
}
