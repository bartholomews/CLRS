use exercises::Exercise;
use crate::binary_search::binary_search;

pub fn ex_2_3_5() -> Exercise {
    let arr = &[31, 41, 51, 52, 60, 65];
    return Exercise {
        number: String::from("2.3-5"),
        question: String::from(
            "Referring back to the searching problem (see Exercise 2.1-3), \
            observe that if the sequence `A` is sorted, we can check the midpoint of the sequence \
            against `v` and eliminate half of the sequence from further consideration. \n\
            The *binary search* algorithm repeats this procedure, halving the size \
            of the remaining portion of the sequence each time. \n\
            Write pseudocode, either iterative or recursive, for binary search. \
            Argue that the worst-case running time of binary search is ϴ(lg n)?"
        ),
        answer: format!(
            "BINARY_SEARCH (A, target, i = 1, j = A.length)\n\
            mid = (i + j) / 2 \n\
            curr = A[mid] \n\
            if(curr = target) return mid \n\
            else if(i = j) return NIL \n\
            else if (curr > target) return BINARY_SEARCH(A, target, i, mid - 1) \n\
            else return BINARY_SEARCH(A, target, mid + 1, j)\n\n\
            --------------------------------------------\n\
            binary_search\n\
            --------------------------------------------\n\
            pub fn binary_search<T: Eq + Ord>(arr: &[T], target: &T, i: usize, j: usize) -> Option<usize> {{ \n\
            \t let mid = (i + j) / 2; \n\
            \t let curr = &arr[mid]; \n\
            \t return if curr == target {{ \n\
            \t\t Some(mid) \n\
            \t }} else if i == j {{ \n\
            \t\t None \n\
            \t }} else if curr > target {{ \n\
            \t\t binary_search(arr, target, i, mid - 1) \n\
            \t }} else {{ \n\
            \t\t binary_search(arr, target, mid + 1, j) \n\
            \t }}; \n\
            }} \n\n\
            [31, 41, 51, 52, 60, 65] \n\
            binary::search(arr, 42, 0, 6) = {:?} \n\
            binary::search(arr, 42, 0, 6) = {:?} \n\
            binary::search(arr, 65, 0, 6) = {:?} \n\n\
            Since we are cutting the array in half on each iteration, at worst case \
            we will stop when the subarray has size 1. \n\
            That is the inverse of exponential growth (i.e. `log n` iterations). \
            For each iteration the cost is just 1 (to check the mid index) \n\
            which gives the final result `lg n`",
            binary_search(arr, &42, 0, 6),
            binary_search(arr, &41, 0, 6),
            binary_search(arr, &65, 0, 6)
        ),
    };
}