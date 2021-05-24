use exercises::Exercise;
use crate::selection_sort;

pub fn ex_2_2_2() -> Exercise {
    let a1 = vec![31, 41, 59, 26, 41, 58];
    let a2 = &mut a1.clone();
    selection_sort::apply(a2);
    return Exercise {
        number: String::from("2.2-2"),
        question: String::from(
            "Consider sorting `n` numbers stored in an array `A` by first finding \
            the smallest element of `A` and exchanging it with the element in `A[1]`. \n\
            Then find the second smallest element of `A`, and exchange it with `A[2]`. \
            Continue in this manner for the first `n-1` elements of `A`. \n\
            Write pseudocode for this algorithm, which is known as *selection sort*. \n\
            What loop invariant does this algorithm maintain ? \n\
            Why does it need to run for only the first `n-1` elements, rather than for all `n` elements ? \n\
            Give the best-case and worst-case running times of selection sort in ϴ-notation."
        ),
        answer: format!(
            "for i = 0 to A.length - 1\n\
            \t s = i\n\
            \t for j = i + 1 to A.length\n\
            \t\t if(A[j] < s) s = j\n\
            \t swap A[i] with A[s]\n\n\
            The loop invariant is similar to the one for insertion sort, \
            that is after each iteration, the sub-array [0..i-1] is always sorted. \n\
            It needs to run for only the first `n-1` elements because \
            the algorithms checks all the elements at each iteration, \n\
            the ones that are left over are always the larger; \
            so if there is only one left, that will be the larger. \n\
            Both best and worst case will run in `Θ(n2)`, since each element \n\
            will be compared with all the other elements - 1\n\n\
            --------------------------------------------\n\
            selection::sort\n\
            pub fn apply<T: Ord>(arr: &mut [T]) {{\n\
            \tfor i in 0..arr.len() - 1 {{\n\
            \t\tlet s = i;\n\
            \t\tfor j in i + 1..arr.len() {{\n\
            \t\t\tif arr[j] < arr[s] {{\n\
            \t\t\t\tarr.swap(j, s)\n\
            \t\t\t}}\n\
            \t\t}}\n\
            \t}}\n\
            }}\n\
            --------------------------------------------\n\
            {:?}\n\
            {:?}", a1, a2
        ),
    };
}