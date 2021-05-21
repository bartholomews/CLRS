use std::str::FromStr;

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use num::BigInt;

use exercises::{Exercise, QA};

use crate::section_02::ex_01::ex_1_2_1;
use crate::section_02::ex_02::ex_1_2_2;
use crate::section_02::ex_03::ex_1_2_3;

mod ex_01;
mod ex_02;
mod ex_03;

pub fn main() {
    let exercises = [ex_1_2_1(), ex_1_2_2(), ex_1_2_3(), ex_1_1()];
    for e in exercises.iter() { e.run(); }
}

fn ex_1_1() -> Exercise {
    const CELL_SIZE: usize = 17;
    fn first_cell(str: String) -> String { format!("{: ^1$}|", str, CELL_SIZE - 9) }
    fn cell(str: String) -> String { format!("{: ^1$}|", str, CELL_SIZE) }
    fn row_separator() -> String { format!("{:-<1$}", "", CELL_SIZE * 8) }

    let running_times_microseconds: [(f64, &str); 7] = [
        (1_000_000.0, "1 second"),
        (60_000_000.0, "1 minute"),
        (3600_000_000.0, "1 hour"),
        (86400_000_000.0, "1 day"),
        (2629800_000_000.0, "1 month"),
        (31557600_000_000.0, "1 year"),
        (3155760000_000_000.0, "1 century")
    ];

    let mut first_row = first_cell(String::from(""));
    for (_, str) in running_times_microseconds.iter() {
        let time: &str = str;
        first_row += cell(String::from(time)).as_str();
    }
    first_row += "\n";
    first_row += row_separator().as_str();

    // lg_n
    let mut lg_n = first_cell(String::from("lg_n"));
    for (time, _) in running_times_microseconds.iter() {
        // we need to determine the largest n such that:
        // log_n ≤ time
        // 2^log_n ≤ 2^time
        // n ≤ 2^time
        let bi = BigInt::from_f64(time.to_f64().unwrap()).unwrap();
        let three_bi = BigInt::from_f64(3.0_f64).unwrap();
        let ten_bi = BigInt::from_f64(10.0).unwrap();
        let res_bi = three_bi * (bi / ten_bi);
        // let lg_n = format!("≈10^({}*10^3)", round);
        if res_bi.to_string().len() <= 10 {
            let res = format!("≈10^{}", res_bi);
            lg_n += cell(res).as_str();
        } else {
            let res = format!("≈10^({}*10^8)", res_bi / 100_000_000);
            lg_n += cell(res).as_str();
        }
    }
    lg_n += "\n";
    lg_n += row_separator().as_str();

    // sqrt_n
    let mut sqrt_n = first_cell(String::from("sqrt_n"));
    for (time, _) in running_times_microseconds.iter() {
        // https://www.chilimath.com/lessons/advanced-algebra/inverse-of-square-root-function/
        // sqrt_n = time
        // (sqrt_n)^2 = time^2 [ (sqrt_n)^2 === n ]
        // n = time^2
        let time_bd = BigDecimal::from_str(time.to_string().as_str()).unwrap();
        let res = time_bd.square();
        if res.digits() > 24 {
            let n = BigDecimal::from_str("100_000_000_000_000_000_000_000_000").unwrap();
            let res = format!("{}*10^26", (res / n).round(4));
            sqrt_n += cell(String::from(res)).as_str();
        } else {
            let n = 100_000_000_000_000_u64;
            let res = format!("{}*10^14", (res / BigDecimal::from_u64(n).unwrap()));
            sqrt_n += cell(String::from(res)).as_str();
        }
    }
    sqrt_n += "\n";
    sqrt_n += row_separator().as_str();

    // n
    let mut n = first_cell(String::from("n"));
    for (time, _) in running_times_microseconds.iter() {
        // https://stackoverflow.com/a/50458236
        let res = format!("{}*10^6", (time.trunc() as usize) / 1_000_000);
        n += cell(res).as_str();
    }
    n += "\n";
    n += row_separator().as_str();

    // n_lg_n
    let mut n_lg_n = first_cell(String::from("n_lg_n"));
    for (time, _) in running_times_microseconds.iter() {
        // https://stackoverflow.com/a/3847435
        fn bisection(time: f64) -> f64 {
            let mut lower: f64 = 0.0;
            let mut upper: f64 = 10_000_000_000_000_000_000.0;
            loop {
                let middle: f64 = (lower + upper) / 2.0;
                if lower == middle || middle == upper {
                    return middle;
                } else if (middle * middle.log2()) > time {
                    upper = middle;
                } else {
                    lower = middle;
                }
            }
        }

        fn calc(time: f64) -> String {
            let res = bisection(time);
            return if res > 10_000_000_f64 {
                let bd = BigDecimal::from_f64(res.trunc() / 100_000_000.0)
                    .unwrap()
                    .round(4);
                format!("{}*10^8", bd)
            } else {
                format!("{}", res.trunc())
            };
        }

        n_lg_n += cell(String::from(calc(time.to_f64().unwrap()))).as_str();
    }
    n_lg_n += "\n";
    n_lg_n += row_separator().as_str();

    // n_squared
    let mut n_squared = first_cell(String::from("n^2"));
    for (time, _) in running_times_microseconds.iter() {
        // https://stackoverflow.com/a/50458236
        let res = format!("{:.4}", time.sqrt() as usize);
        n_squared += cell(res).as_str();
    }
    n_squared += "\n";
    n_squared += row_separator().as_str();

    // n_cubed
    let mut n_cubed = first_cell(String::from("n^3"));
    for (time, _) in running_times_microseconds.iter() {
        // https://stackoverflow.com/a/50458236
        let res = format!("{:.4}", time.cbrt() as usize);
        n_cubed += cell(res).as_str();
    }
    n_cubed += "\n";
    n_cubed += row_separator().as_str();

    // two_pow_n
    let mut two_pow_n = first_cell(String::from("2^n"));
    for (time, _) in running_times_microseconds.iter() {
        // https://www.purplemath.com/modules/solvexpo2.htm
        // 2^n = time
        // log2(2^n) = log2(time)   [ log_b(x^e) = e * log_b(x) ]
        // n * log2(2) = log2(time)
        // n = log2(time) / log2(2)
        let calc = (time.log2() / (2.0 as f64).log2()) as usize;
        let res = format!("{:.4}", calc);
        two_pow_n += cell(res).as_str();
    }
    two_pow_n += "\n";
    two_pow_n += row_separator().as_str();

    // fact_n!
    let mut fact_n = first_cell(String::from("n!"));
    for (time, _) in running_times_microseconds.iter() {
        // https://stackoverflow.com/a/50458236
        // let fact_n = format!("{:.4}", time.cbrt() as usize);
        fn fact(n: f64) -> f64 {
            if n <= 1.0 { 1.0 } else { n * fact(n - 1.0) }
        }

        // n! = time
        let mut curr: f64 = 1.0;
        let mut res = fact(curr);
        while res < time.trunc() {
            curr += 1.0;
            res = fact(curr);
        }

        let res = format!("{:.4}", (curr - 1.0) as usize);
        fact_n += cell(String::from(res)).as_str();
    }
    fact_n += "\n";
    fact_n += row_separator().as_str();

    return Exercise {
        number: String::from("1.1"),
        question: String::from(
            "For each function `f(n)` and time `t` in the following table,\n\
            determine the largest size `n` of a problem that can be solved in time `t`,\n\
            assuming that the algorithm to solve the problem takes `f(n)` microseconds."
        ),
        answer: format!(
            "Reference:\n\
            https://udel.edu/~caviness/Class/CISC320-02S/exercise-solns/ch01/R-1.7.pdf\n\
            {}\n\
            {}\n\
            {}\n\
            {}\n\
            {}\n\
            {}\n\
            {}\n\
            {}\n\
            {}",
            row_separator(),
            first_row,
            lg_n,
            n,
            n_lg_n,
            n_squared,
            n_cubed,
            two_pow_n,
            fact_n
        ),
    };
}
