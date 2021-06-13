/*
    "Combine" step of merge-sort: merge two sorted sub-sequences;
    `p`, `q` and `r` are indices into `arr` such that `p <= q < r`.
    The procedure assumes that the sub-arrays A[p..q] and A[q+1..r] are in sorted order.
    It *merges* them to form a single sorted subarray that replaces the current subarray A[p..r]
*/
fn merge(arr: &mut[u32], p: usize, q: usize, r:usize) {
    let mut left_arr = Vec::from(&arr[p..q]);
    left_arr.push(u32::MAX);
    let mut right_arr = Vec::from(&arr[q..r]);
    right_arr.push(u32::MAX);
    let mut l_i = 0;
    let mut r_i = 0;
    for k in p..r {
        if left_arr[l_i] <= right_arr[r_i] {
            arr[k] = left_arr[l_i];
            l_i += 1;
        } else {
            arr[k] = right_arr[r_i];
            r_i += 1;
        }
    }
}

pub fn apply(arr: &mut [u32], p: usize, r: usize) {
    if (r - p) >= 2 { // range of size 1 is already sorted (base case)
        let q = (p + r) / 2;
        apply(arr, p, q);
        apply(arr, q, r);
        let mut y: Vec<u32> = arr.to_vec();
        // merge2(&arr[0..m], &arr[m..size], &mut y[..]);
        merge(&mut y[..], p, q, r);
        arr.copy_from_slice(&y);
    }
}

fn merge2<T: Copy + PartialOrd>(left: &[T], right: &[T], target: & mut[T]) {
    let mut i = 0;
    let mut l_i = 0;
    let mut r_i = 0;
    while l_i < left.len() && r_i < right.len() {
        if left[l_i] < right[r_i] {
            target[i] = left[l_i];
            l_i += 1;
        } else {
            target[i] = right[r_i];
            r_i += 1;
        }
        i += 1;
    }
    if l_i < left.len() { target[i..].copy_from_slice(&left[l_i..]); }
    if r_i < left.len() { target[i..].copy_from_slice(&right[r_i..]); }
}

// https://www.hackertouch.com/merge-sort-in-rust.html
pub fn apply2<T: Copy + PartialOrd>(arr: &mut [T]) {
    let size = arr.len();
    if size > 1 {
        let m = size / 2;
        apply2(&mut arr[0..m]);
        apply2(&mut arr[m..size]);
        let mut y: Vec<T> = arr.to_vec();
        merge2(&arr[0..m], &arr[m..size], &mut y[..]);
        arr.copy_from_slice(&y);
    }
}