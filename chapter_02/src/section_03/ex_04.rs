use exercises::Exercise;
use crate::insertion_sort;

pub fn ex_2_3_4() -> Exercise {
    let x = vec!(3, 41, 52, 26, 38, 57, 9, 49);
    let mut y = x.to_vec();
    insertion_sort::recursive(&mut y, x.len() - 1);
    return Exercise {
        number: String::from("2.3-4"),
        question: String::from(
            "We can express insertion sort as a recursive procedure as follows. \n\
            In order to sort A[1..n], we recursively sort A[1..n-1] \
            and then insert A[n] into the sorted array A[1..n-1]. \n\
            Write a recurrence for the worst-case running time of this recursive version \
            of insertion sort."
        ),
        answer: format!(
            "https://atekihcan.github.io/CLRS/02/E02.03-04/\n\
            \t\t{{ Θ(1)          [if n = 1]\n\
            T(n) =  {{\n\
            \t\t{{T(n-1) + Θ(n)  [if n > 1]\n\
            \n\
            For n = 1, the array is already sorted.\n\
            For n > 1, the time it takes to sort n-1 elements [T(n-1)], \
            plus Θ(n) for shifting all the elements \n\
            (at worst case it would be ordered in reverse).\n\n\
            ---------------------------------------------------\n\
            insertion::sort::recursive\n\
            ---------------------------------------------------\n\
            pub fn recursive<T: Ord>(arr: &mut[T], j: usize) {{\n\
            \tif j > 0 {{ \n\
            \t\trecursive(arr, j - 1);\n\
            \t\tlet mut i: usize = j - 1;\n\
            \t\twhile i > 0 && arr[i] > arr[i+1] {{\n\
            \t\t\tarr.swap(i, i+1);\n\
            \t\t\ti -= 1;\n\
            \t\t}}\n\
            \t}}\n\
            }}\n\n\
            {:?}\n{:?}",
            x, y
        ),
    };
}