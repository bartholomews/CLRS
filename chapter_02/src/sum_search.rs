use crate::merge_sort;

pub fn sum_search(arr: &mut[u32], target: u32) -> bool {
    merge_sort::apply(arr, 0, arr.len());
    for i in 0..arr.len() {
        let n = arr[i];
        let res = binary_search(arr, n, target, 0, arr.len());
        match res {
            None => {}
            Some(res) => {
                if res != i { return true; }
            }
        }
    }
    return false
}

fn binary_search(arr: &[u32], n: u32, target: u32, i: usize, j: usize) -> Option<usize> {
    let mid = (i + j) / 2;
    let curr = &arr[mid] + n;
    return if curr == target {
        Some(mid)
    } else if i == j {
        None
    } else if curr > target {
        binary_search(arr, n, target, i, mid - 1)
    } else {
        binary_search(arr, n, target, mid + 1, j)
    };
}