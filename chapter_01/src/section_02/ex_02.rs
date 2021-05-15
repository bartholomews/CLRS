use crate::Exercise;

pub fn ex_1_2_2() -> Exercise {
    fn calc_insertion_sort(n: f64) -> f64 { return 8.0 * n.exp2(); }
    fn calc_merge_sort(n: f64) -> f64 { return 64.0 * n * n.log(2.0); }
    fn calc(n: f64) {
        println!("n: {}", n);
        let insert = calc_insertion_sort(n);
        let merge = calc_merge_sort(n);
        if insert < merge {
            println!("INSERT WINS! {} vs {}", insert, merge)
        } else {
            println!("MERGE WINS! {} vs {}", merge, insert)
        }
    }

    calc(4.0);
    calc(6.0);
    calc(7.0);
    calc(64.0);

    return Exercise {
        number: String::from("1.2-2"),
        question: String::from(
            "Suppose we are comparing implementations of insertion sort and merge sort on the same machine.\n\
            For inputs of size `n`, insertion sort runs in `8n^2` steps, while merge sort runs in `64n lg n` steps.\n\
            For which values of `n` does insertion sort beat merge sort ?"
        ),
        answer: String::from(
            "For `n` == 4.0, insert beats merge 128 to 512\n\
            For `n` == 4.0, insert beats merge 128 to 512\n\
            For `n` == 6.0, insert beats merge 512 to ~992\n\
            For `n` == 7.0, insert beats merge 1024 to ~1257\n\
            For `n` == 8.0, merge beats insert 1536 to 2048\n\
            In general, insertion sorts beats merge sort for values of `n` < 7.0"
        ),
    };
}
