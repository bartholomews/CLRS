use crate::Exercise;

pub fn ex_1_1_5() -> Exercise {
    return Exercise {
        number: String::from("1.1-5"),
        question: String::from(
            "Come up with a real-world problem in which only the best solution will do.\n\
            Then come up with one in which a solution that is \"approximately\" the best is good enough."),
        answer: String::from(
            "The problem of target tracking requires the best solution \
            in mission-critical applications such as naval systems;\n\
            other systems such as user location tracking for a street map view \
            can afford less precision without critical consequences."
        ),
    };
}