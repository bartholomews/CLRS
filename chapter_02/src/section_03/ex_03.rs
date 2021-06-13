use exercises::Exercise;

pub fn ex_2_3_3() -> Exercise {
    return Exercise {
        number: String::from("2.3-3"),
        question: String::from(
            "Use mathematical induction to show that when `n` is an exact power of 2, \
            the solution of the recurrence \n\n\
            \t\t{ 2                if n = 2\n\
            T(n) =  {\n\
            \t\t{ 2T(n/2) + n      if n = 2^k, for k > 1\n\n\
            is `T(n) = n lg n`"
        ),
        answer: format!(
            "https://atekihcan.github.io/CLRS/02/E02.03-03/ \n\n\
             BASE CASE:\n\
             Proof for [n = 2]\n\
             `T(2) = 2` \n\
             `2 lg 2 = 2 * 1 = 2`\n\n\
             INDUCTIVE STEP:\n\
             Assumption: [T(2^k) = 2^k lg 2^k]\n\
             Proof for [n = k + 1]\n\
             `2T(2^(k+1) / 2) + 2^(k+1)`        [2^(k+1) = 2 * 2^k] \n\
             `2T(2 * 2^k / 2) + 2 * 2^k`\n\
             `2T(2^k) + 2 * 2^k`\n\
             `2 * 2^k lg 2^k + 2 * 2^k`         [T(2^k) = 2^k lg 2^k]\n\
             `2 * 2^k (lg 2^k + 1)              [a * b + a * c = a (b + c)]`\n\
             `2^(k+1) (lg 2^k + lg 2)`          [1 = lg 2]\n\
             `2^(k+1) lg 2^(k+1)`"
        ),
    };
}