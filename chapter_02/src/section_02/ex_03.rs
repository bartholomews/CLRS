use exercises::Exercise;

pub fn ex_2_2_3() -> Exercise {
    return Exercise {
        number: String::from("2.2-3"),
        question: String::from(
            "Consider linear search again (see Exercise 2.1-3). \
            How many elements of the input sequence need to be checked on the average, \n\
            assuming that the element being searched for is equally likely to be any element in the array? \n\
            How about in the worst case? What are the average-case and worst-case running times \
            of linear search in ϴ-notation? \n\
            Justify your answers."
        ),
        answer: format!(
            "Average-case: ϴ(n/2)\n\
            Worst-case: ϴ(n)\n\
            Best-case: ϴ(1)"
        ),
    };
}