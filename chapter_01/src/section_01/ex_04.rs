use crate::Exercise;

pub fn ex_1_1_4() -> Exercise {
    return Exercise {
        number: String::from("1.1-4"),
        question: String::from(
            "How are the shortest-path and traveling-salesman problems given above similar? \
        How are they different?",
        ),
        answer: String::from(
            "Both TSP and shortest-path needs to visit the less number of nodes (or with smallest weight) \
        in a graph in order to go from node A to node B.\n\
        But the TSP needs to visit ALL nodes (NP-complete) \
        while with the shortest-path there's no need to (polynomial-time)."),
    };
}