use crate::Exercise;

pub fn ex_1_2_1() -> Exercise {
    return Exercise {
        number: String::from("1.2-1"),
        question: String::from(
            "Give an example of an application that requires algorithmic content \
            at the application level, and discuss the function of the algorithms involved."
        ),
        answer: String::from(
            "A search engine needs to sort results based on some kind of ranking system;\n\
            a large number of results might require a sorting algorithm that can scale well \
            (i.e. logarithmically instead of quadratically)"
        ),
    };
}
