use exercises::Exercise;
use crate::sum_search::sum_search;

pub fn ex_2_3_7() -> Exercise {
    let arr1 = &mut[5, 10, 6, 2, 3, 8];
    let arr2 = &mut[5, 10, 6, 2, 3, 9];
    return Exercise {
        number: String::from("2.3-7"),
        question: String::from(
            "Describe a ϴ(n lg n)-time algorithm that, given a set `S` of `n` integers \
            and another integer `x`, \n\
            determines whether or not there exist two elements \
            in `S` whose sum is exactly `x`."
        ),
        answer: format!(
            "We can use merge-sort to sort the array in `n lg n`, \
            then for each element apply binary search on the sorted array, \n\
            to find the other element which sums up to `n`, \
            which again would run in `n lg n` worst-case. That would give a `Θ(n lg n)` time. \n\
            -----------------------------------------------------------\n\
            sum_search\n\
            -----------------------------------------------------------\n\
             pub fn sum_search(arr: &mut[u32], target: u32) -> bool {{ \n\
             \t merge_sort::apply(arr, 0, arr.len()); \n\
             \t for i in 0..arr.len() {{ \n\
             \t\t let n = arr[i]; \n\
             \t\t let res = binary_search(arr, n, target, 0, arr.len()); \n\
             \t\t match res {{ \n\
             \t\t\t None => {{}} \n\
             \t\t\t Some(res) => {{ \n\
             \t\t\t\t if res != i {{ return true; }} \n\
             \t\t\t }} \n\
             \t\t }} \n\
             \t }} \n\
             \t return false \n\
             }} \n\n\
             fn binary_search(arr: &[u32], n: u32, target: u32, i: usize, j: usize) -> Option<usize> {{ \n\
             \t let mid = (i + j) / 2; \n\
             \t let curr = &arr[mid] + n; \n\
             \t return if curr == target {{ \n\
             \t\t Some(mid) \n\
             \t }} else if i == j {{ \n\
             \t\t None \n\
             \t }} else if curr > target {{ \n\
             \t\t binary_search(arr, n, target, i, mid - 1) \n\
             \t }} else {{ \n\
             \t\t binary_search(arr, n, target, mid + 1, j) \n\
             \t }}; \n\
             }} \n\n\
             {:?} (n = 10) => {:?} \n\
             {:?} (n = 10) => {:?}
            ",
            arr1.to_vec(), sum_search(arr1, 10),
            arr2.to_vec(), sum_search(arr2, 10)
        ),
    };
}