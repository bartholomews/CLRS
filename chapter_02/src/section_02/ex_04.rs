use exercises::Exercise;

pub fn ex_2_2_4() -> Exercise {
    return Exercise {
        number: String::from("2.2-4"),
        question: String::from(
            "How can we modify almost any algorithm to have a good best-case running time?"
        ),
        answer: String::from(
            "With an ad-hoc condition to check if that is a best-case. \n\
            E.g. any sorting algorithm will run in `Θ(n)` with that check."),
    };
}