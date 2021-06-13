use exercises::Exercise;
use crate::merge_sort;

pub fn ex_2_3_2() -> Exercise {
    let x = vec!(3, 41, 52, 26, 38, 57, 9, 49);
    let mut y = x.to_vec();
    merge_sort::apply2(&mut y);
    return Exercise {
        number: String::from("2.3-2"),
        question: String::from(
            "Rewrite the MERGE procedure so that it does not use sentinels, \
            instead stopping once either array L or R has had all its elements copied back to A \
            and then copying the remainder of the other array back into A."
        ),
        answer: format!(
            "--------------------------------------------\n\
            merge::sort\n\
            --------------------------------------------\n\
            fn merge2<T: Copy + PartialOrd>(left: &[T], right: &[T], target: & mut[T]) {{\n\
            \tlet mut i = 0;\n\
            \tlet mut l_i = 0;\n\
            \tlet mut r_i = 0;\n\
            \twhile l_i < left.len() && r_i < right.len() {{\n\
            \t\tif left[l_i] < right[r_i] {{\n\
            \t\t\ttarget[i] = left[l_i];\n\
            \t\t\tl_i += 1;\n\
            \t\t}} else {{\n\
            \t\t\ttarget[i] = right[r_i];\n\
            \t\t\tr_i += 1;\n\
            \t\t}}\n\
            \t\ti += 1;\n\
            \t}}\n\
            \tif l_i < left.len() {{ target[i..].copy_from_slice(&left[l_i..]); }}\n\
            \tif r_i < left.len() {{ target[i..].copy_from_slice(&right[r_i..]); }}\n\
            }}\n\n\
            // https://www.hackertouch.com/merge-sort-in-rust.html\n\
            pub fn apply2<T: Copy + PartialOrd>(arr: &mut [T]) {{\n\
            \tlet size = arr.len();\n\
            \tif size > 1 {{\n\
            \t\tlet m = size / 2;\n\
            \t\tapply2(&mut arr[0..m]);\n\
            \t\tapply2(&mut arr[m..size]);\n\
            \t\tlet mut y: Vec<T> = arr.to_vec();\n\
            \t\tmerge2(&arr[0..m], &arr[m..size], &mut y[..]);\n\
            \t\tarr.copy_from_slice(&y);\n\
            \t}}\n\
            }}\n\
            --------------------------------------------\n\
            {:?}\n{:?}", x, y
        ),
    };
}