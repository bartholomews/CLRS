use crate::insertion_sort;
use exercises::Exercise;

pub fn ex_2_1_1() -> Exercise {
    let a1 = &mut vec![31, 41, 59, 26, 41, 58];
    let a2 = &mut a1.clone();
    insertion_sort::inc(a2);
    return Exercise {
        number: String::from("2.1-1"),
        question: String::from(
            "Illustrate the operation of INSERTION-SORT on the array A = {31,41,59,26,41,58}"
        ),
        answer: format!(
            "[31, 41, 59, 26, 41, 58]\n\
             [31, (41), 59, 26, 41, 58]\n\
             [31, 41, (59), 26, 41, 58]\n\
             [31, 41, 59, <- (26), 41, 58]\n\
             [(26), 31 -> 41 -> 59 -> |, 41, 58]\n\
             [26, 31, 41, 59, <- (41), 58]\n\
             [26, 31, 41, (41), 59 -> |, 58]\n\
             [26, 31, 41, 41, 59, <- (58)]\n\
             [26, 31, 41, 41, (58), 59 -> |)]\n\n\
            --------------------------------------------\n\
            insertion::sort\n\
            --------------------------------------------\n\
            pub fn inc<T: Ord>(arr: &mut [T]) {{\n\
            \tfor i in 1..arr.len() {{\n\
            \t\tlet mut j = i;\n\
            \t\twhile j > 0 && arr[j - 1] > arr[j] {{\n\
            \t\t\tarr.swap(j - 1, j);\n\
            \t\t\tj -= 1;\n\
            \t\t}}\n\
            \t}}\n\
            }}\n\
            --------------------------------------------\n\
            {:?}\n\
            {:?}", a1, a2
        ),
    };
}