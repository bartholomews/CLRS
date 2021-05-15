use crate::Exercise;

pub fn ex_1_1_3() -> Exercise {
    return Exercise {
        number: String::from("1.1-3"),
        question: String::from(
            "Select a data structure that you have seen previously, \
        and discuss its strengths and limitations.",
        ),
        answer: String::from(
            "A linked list is good at appending or deleting the first element Θ(1), \
            but it doesn't scale well at indexing Θ(n)."
        ),
    };
}