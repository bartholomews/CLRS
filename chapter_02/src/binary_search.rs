pub fn binary_search<T: Eq + Ord>(arr: &[T], target: &T, i: usize, j: usize) -> Option<usize> {
    let mid = (i + j) / 2;
    let curr = &arr[mid];
    return if curr == target {
        Some(mid)
    } else if i == j {
        None
    } else if curr > target {
        binary_search(arr, target, i, mid - 1)
    } else {
        binary_search(arr, target, mid + 1, j)
    };
}