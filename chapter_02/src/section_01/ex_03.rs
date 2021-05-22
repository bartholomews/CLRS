use exercises::Exercise;

pub fn ex_2_1_3() -> Exercise {
    fn find<T: Eq>(xs: Vec<T>, v: T) -> Option<usize> {
        for i in 0..xs.len() {
            if xs[i] == v { return Some(i); } else { continue; }
        }
        return None;
    }

    return Exercise {
        number: String::from("2.1-3"),
        question: String::from(
            "Consider the *searching problem*:\n\
            *Input* - A sequence of `n` numbers A = { a_1, a_2, ..., a_n } \n\
            *Output* - An index `i` such that v = A[i] \
            or the special value NIL if `v` does not appear in A.\n\n\
            Write pseudocode for *linear search*, which scans through the sequence, looking for `v`.\n\
            Using a loop invariant, prove that your algorithm is correct.\n\
            Make sure that your loop invariant fulfills the three necessary steps."
        ),
        answer: format!(
            "for j = 0 to A.length\n\
            \t if (A[j] = v) return j;\n\
            return NIL\n\n\
            Initialization: before the loop iteration, j == 0 and the checked subarray is empty;
            \ttherefore, `v` cannot appear in A. The base case is then NIL.\n\n\
            Maintenance: before each iteration, the checked subarray will maintain the loop invariant
             since there is no state change unless the clause `A[j] = v` is true,
             but in that case the loop terminates.\n\n\
            Termination: After all the array has been checked the loop terminates naturally
             and it maintain the loop invariant of NIL. The \"special\" termination condition
             is when the clause `A[j] = v` is true, in that case the output will have the expected index.\n\n\
             v = 4; seq = [1,2,3]; res = {:?}\n\
             v = 2; seq = [1,2,3]; res = {:?}",
            find(vec![1,2,3], 4), find(vec![1, 2, 3], 2)
        ),
    };
}