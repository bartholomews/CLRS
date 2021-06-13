use exercises::Exercise;
use crate::insertion_sort;

pub fn ex_2_1_2() -> Exercise {
    let a1 = vec![31, 41, 59, 26, 41, 58];
    let a2 = &mut a1.clone();
    insertion_sort::dec(a2);
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
            }}\n\n\
            --------------------------------------------\n\
            {:?}\n\
            {:?}", a1, a2
        ),
    };
}