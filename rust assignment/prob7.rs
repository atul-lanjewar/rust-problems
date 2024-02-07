fn kth_smallest_heap(arr: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > arr.len() {
        return None;
    }

    let mut heap = BinaryHeap::new();
    for num in arr.iter().take(k) {
        heap.push(*num);
    }

    for num in arr.iter().skip(k) {
        if *num < heap.peek().unwrap() {
            heap.pop();
            heap.push(*num);
        }
    }

    heap.pop()
}