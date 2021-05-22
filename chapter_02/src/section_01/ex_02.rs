use exercises::Exercise;
use crate::insertion;

pub fn ex_2_1_2() -> Exercise {
    let arr = &mut vec![31, 41, 59, 26, 41, 58];
    insertion::sort_decreasing(arr);
    return Exercise {
        number: String::from("2.1-2"),
        question: String::from(
            "Rewrite the INSERTION-SORT procedure to sort into non-increasing \
            instead of non-decreasing order."
        ),
        answer: format!(
            "for i in 1..arr.len() {{\n\
             \t let mut j = i;\n\
             \t while j > 0 && arr[j - 1] < arr[j] {{\n\
             \t\t arr.swap(j - 1, j);\n\
             \t\t j -= 1;\n\
             \t }}\n\
            }}\n
            \n{:?}", arr
        ),
    };
}