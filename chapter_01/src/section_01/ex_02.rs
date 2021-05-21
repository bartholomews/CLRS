use exercises::Exercise;

pub fn ex_1_1_2() -> Exercise {
    return Exercise {
        number: String::from("1.1-2"),
        question: String::from(
            "Other than speed, what other measures of efficiency might one use in a real-world setting?"
        ),
        answer: String::from("Correctness, resilience, memory."),
    };
}