use exercises::Exercise;

use crate::binary_addition::binary_addition;

pub fn ex_2_1_4() -> Exercise {
    // let a = vec![1, 0, 1, 1];
    // let b = vec![1, 0, 1, 0];
    let a = &vec![true, false, true, true];
    let b = &vec![true, false, true, false];
    fn to_int(x: &Vec<bool>) -> Vec<i8> {
        return x.iter().map(|&b| b as i8).collect();
    }
    // let c = vec![1, 0, 1, 0, 1];
    return Exercise {
        number: String::from("2.1-2"),
        question: String::from(
            "Consider the problem of adding two `n-bit` binary integers, \
            stored in two n-element arrays A and B. \n\
            The sum of the two integers should be stored in binary form in an (n + 1)-element array C.\n\
            State the problem formally and write pseudocode for adding the two integers."
        ),
        answer: format!(
            "This problem consists of binary addition over arrays.\n\
            The two arrays have the same size so the two binary values at the same index \
            can be summed as single-digit binary addition,\n\
            carrying from right to left.\n\n\

            e.g.\n\
            {:?} +\n\
            {:?} =\n\
            {:?}\n\n\

            BINARY ADDITION(A, B) \n\
            C = []\n\
            carry = 0 \n\
            for j = A.length to 0\n\
            \t// (0 + 0 = carry, then carry must be 0)\n\
            \tif(A[j] = 0 and B[j] = 0)\n\
            \t\tC[j+1] = carry\n\
            \t\tcarry = 0\n\
            \t// (1 + 1 = carry, then carry must be 1)\n\
            \tif(A[j] = 1 and B[j] = 1)\n\
            \t\tC[j+1] = carry\n\
            \t\tcarry = 1\n\
            \t// (0 + 1 = !carry, then carry is unchanged)\n\
            \telse C[j+1] = !carry\n\
            C[0] = carry", to_int(&a), to_int(&b), binary_addition(a, b)
        ),
    };
}