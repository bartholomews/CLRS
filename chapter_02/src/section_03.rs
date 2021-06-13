use exercises::QA;

use crate::section_03::ex_01::ex_2_3_1;
use crate::section_03::ex_02::ex_2_3_2;
use crate::section_03::ex_03::ex_2_3_3;
use crate::section_03::ex_04::ex_2_3_4;
use crate::section_03::ex_05::ex_2_3_5;
use crate::section_03::ex_06::ex_2_3_6;
use crate::section_03::ex_07::ex_2_3_7;

mod ex_01;
mod ex_02;
mod ex_03;
mod ex_04;
mod ex_05;
mod ex_06;
mod ex_07;

pub fn main() {
    let exercises = [
        ex_2_3_1(),
        ex_2_3_2(),
        ex_2_3_3(),
        ex_2_3_4(),
        ex_2_3_5(),
        ex_2_3_6(),
        ex_2_3_7()
    ];
    for e in exercises.iter() { e.run(); }
}