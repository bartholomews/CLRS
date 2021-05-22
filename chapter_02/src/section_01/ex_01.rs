use crate::insertion;
use exercises::Exercise;

pub fn ex_2_1_1() -> Exercise {
    let arr = &mut vec![31, 41, 59, 26, 41, 58];
    insertion::sort(arr);
    return Exercise {
        number: String::from("2.1-1"),
        question: String::from(
            "Illustrate the operation of INSERTION-SORT on the array A = {31,41,59,26,41,58}"
        ),
        answer: format!(
            "The algorithm will start the first iteration with A[j] = `41`.\n\
          It will exit the while loop at the second conditional \
          since 31 is not greater than 41, re-assigning the current key to the same initial index.\n\
          On the second iteration, the same will happen since 41 is not greater than 59.\n\
          On the third iteration, it will enter the while loop three times, \
          since all the previous entries are greater than `59`. \n\
          On each step in the while loop the previous entry will be shifted forward: \
          59 to index 3, 41 to index 2, 31 to index 1,\n\
          then will exit the while loop at the first conditional branch \
          since the first index of the array has been reached. \n\
          That index is then assigned to `26`, so that A = {{26, 31, 41, 59, 41, 58}}.\n\
          On the fourth iteration, it will enter the while loop once, where 59 > 41, shifting 59 forward. \n\
          It will then exit the while loop at the second conditional since 41 is not greater than 41.\n\
          On the fifth iteration, it will enter the while loop once, where 59 is greater than 58, shifting 59 forward.\n\
          It will then exit the while loop at the second conditional \
          since 41 is not greater than 58.\n\
          The loop terminates with {:?}.
          ", arr
        ),
    };
}