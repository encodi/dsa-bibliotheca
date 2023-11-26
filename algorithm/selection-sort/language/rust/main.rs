fn selectionSort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut min = i;
        for j in i + 1..len {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        arr.swap(i, min);
    }
}

fn main() {
    let mut arr = [1, 5, 2, 4, 3];
    selectionSort(&mut arr);
    println!("{:?}", arr);
}
