fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = (low + high) / 2;
        if arr[mid] < target {
            low = mid + 1;
        } else if arr[mid] > target {
            high = mid;
        } else {
            // Found the target, check if there are earlier occurrences
            while mid > 0 && arr[mid - 1] == target {
                mid -= 1;
            }
            return Some(mid);
        }
    }

    None
}

let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

println!("Index of 5: {:?}", find_first_occurrence(&arr, 5));  // Output: Some(4)
println!("Index of 11: {:?}", find_first_occurrence(&arr, 11)); // Output: None
println!("Index of 1: {:?}", find_first_occurrence(&arr, 1));  // Output: Some(0)