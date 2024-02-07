//kadane's algorithm

fn max_subarray_kadane(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for num in arr.iter().skip(1) {
        current_sum = std::cmp::max(*num, current_sum + *num);
        max_sum = std::cmp::max(max_sum, current_sum);
    }

    max_sum
}