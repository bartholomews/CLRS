use exercises::Exercise;
use crate::merge_sort;

pub fn ex_2_3_1() -> Exercise {
    let x = vec!(3, 41, 52, 26, 38, 57, 9, 49);
    let mut y = x.to_vec();
    merge_sort::apply(&mut y, 0, 8);
    return Exercise {
        number: String::from("2.3-1"),
        question: String::from(
            "Illustrate the operation of MERGE_SORT on the array A = {3,41,52,26,38,57,9,49}"
        ),
        answer: format!(
            "[3, 41, 52, 26, 38, 57, 9, 49]\n\
             [3, 41, 52, 26] [38, 57, 9, 49]\n\
             [3, 41] [52, 26] [38, 57] [9, 49]\n\
             [3] [41] [52] [26] [38] [57] [9] [49]\n\
             [3, 41] [26, 52] [38, 57] [9, 49]\n\
             [3, 26, 41, 52] [9, 38, 49, 57]\n\
             [3, 9, 26, 38, 41, 49, 52, 57]\n\n\
            --------------------------------------------\n\
            merge::sort\n\
            --------------------------------------------\n\
            fn merge(arr: &mut[u32], p: usize, q: usize, r:usize) {{\n\
            \tlet mut left_arr = Vec::from(&arr[p..q]);\n\
            \tleft_arr.push(u32::MAX);\n\
            \tlet mut right_arr = Vec::from(&arr[q..r]);\n\
            \tright_arr.push(u32::MAX);\n\
            \tlet mut l_i = 0;\n\
            \tlet mut r_i = 0;\n\
            \tfor k in p..r {{\n\
            \t\tif left_arr[l_i] <= right_arr[r_i] {{\n\
            \t\t\tarr[k] = left_arr[l_i];\n\
            \t\t\tl_i += 1;\n\
            \t\t}} else {{\n\
            \t\t\tarr[k] = right_arr[r_i];\n\
            \t\t\tr_i += 1;\n\
            \t\t}}\n\
            \t}}\n\
            }}\n\n\
            pub fn apply(arr: &mut [u32], p: usize, r: usize) {{\n\
            \tif (r - p) >= 2 {{ // range of size 1 is already sorted (base case)\n\
            \t\tlet q = (p + r) / 2;\n\
            \t\tapply(arr, p, q);\n\
            \t\tapply(arr, q, r);\n\
            \t\tlet mut y: Vec<u32> = arr.to_vec();\n\
            \t\tmerge(&mut y[..], p, q, r);\n\
            \t\tarr.copy_from_slice(&y);\n\
            \t}}\n\
            }}\n\
            --------------------------------------------\n\
            {:?}\n\
            {:?}", x, y
        ),
    };
}