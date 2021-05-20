use crate::Exercise;

pub fn ex_1_2_2() -> Exercise {
    fn calc_insertion_sort(n: f64) -> f64 { return 8.0 * n.exp2(); }
    fn calc_merge_sort(n: f64) -> f64 { return 64.0 * n * n.log(2.0); }
    fn calc(n: f64) -> String {

        fn f(x: f64) -> String {
            let trunc = x.trunc();
            if trunc == x { format!("{}", trunc) }
            else { format!("~{}", trunc) }
        }

        let insert = calc_insertion_sort(n);
        let merge = calc_merge_sort(n);
        if insert < merge {
            format!("For `n` == {}, insert beats merge {} to {}", n, f(insert), f(merge))
        } else {
            format!("For `n` == {}, merge beats insert {} to {}", n, f(merge), f(insert))
        }
    }

    return Exercise {
        number: String::from("1.2-2"),
        question: String::from(
            "Suppose we are comparing implementations of insertion sort and merge sort on the same machine.\n\
            For inputs of size `n`, insertion sort runs in `8n^2` steps, while merge sort runs in `64n lg n` steps.\n\
            For which values of `n` does insertion sort beat merge sort ?"
        ),
        answer: format!(
            "fn calc_insertion_sort(n: f64) -> f64 {{ return 8.0 * n.exp2(); }} \n\
            fn calc_merge_sort(n: f64) -> f64 {{ return 64.0 * n * n.log(2.0); }} \n\
            \n\
            {} \n\
            {} \n\
            {} \n\
            {} \n\
            \n\
            In general, insertion sorts beats merge sort for values of `n` < 7.0",
            calc(4.0),
            calc(6.0),
            calc(7.0),
            calc(8.0),
        ),
    };
}
