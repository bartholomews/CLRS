use exercises::Exercise;

pub fn ex_2_3_6() -> Exercise {
    return Exercise {
        number: String::from("2.3-6"),
        question: String::from(
            "Observe that the *while* loop of INSERTION-SORT procedure in Section 2.1 \
            uses a linear search to scan (backward) \n\
            through the sorted subarray A[1..j-1]. \
            Can we use a binary search (see Exercise 2.3-5) instead \n\
            to improve the overall worst-case running time of insertion sort in ϴ(n lg n)?"
        ),
        answer: format!(
            "No, binary search would find the proper position for insertion in `lg n`, \n\
            but we would still need to shift all the elements greater than the one to insert, \
            which will run at Θ(n) at worst-case."
        ),
    };
}