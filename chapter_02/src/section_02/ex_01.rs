use exercises::Exercise;

pub fn ex_2_2_1() -> Exercise {
    return Exercise {
        number: String::from("2.2-1"),
        question: String::from(
            "Express the function `n^3/1000 - 100n^2 - 100n + 3` in terms of ϴ-notation."
        ),
        answer: String::from("Θ(n3) - since the lower order terms and constants are ignored."),
    };
}