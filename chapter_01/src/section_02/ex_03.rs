use crate::Exercise;

pub fn ex_1_2_3() -> Exercise {
    let mut n: u32 = 1;
    let two: u32 = 2;
    while 100 * n.pow(2) > two.pow(n) {
        n += 1;
    }

    return Exercise {
        number: String::from("1.2-3"),
        question: String::from(
            "What is the smallest value of `n` such that an algorithm whose running time is `100n^2`\n\
            runs faster than an algorithm whose running time is `2^n` on the same machine?"
        ),
        answer: format!("The smallest value of `n` such that `100n^2` is faster than `2^n` is {}.", n)
    };
}