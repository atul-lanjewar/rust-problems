fn get_median(arr: &[i32]) -> Option<f64> {
    if arr.is_empty() {
        return None;
    }

    let mid = arr.len() / 2;
    if arr.len() % 2 == 0 {
        // even length, average the middle two elements
        return Some((arr[mid - 1] as f64 + arr[mid] as f64) / 2.0);
    } else {
        // odd length, return the middle element
        return Some(arr[mid] as f64);
    }
}