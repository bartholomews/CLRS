use exercises::QA;

use crate::section_01::ex_01::ex_1_1_1;
use crate::section_01::ex_02::ex_1_1_2;
use crate::section_01::ex_03::ex_1_1_3;
use crate::section_01::ex_04::ex_1_1_4;
use crate::section_01::ex_05::ex_1_1_5;

mod ex_02;
mod ex_01;
mod ex_03;
mod ex_04;
mod ex_05;

pub fn main() {
    let exercises = [ex_1_1_1(), ex_1_1_2(), ex_1_1_3(), ex_1_1_4(), ex_1_1_5()];
    for e in exercises.iter() { e.run(); }
}