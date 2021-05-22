use exercises::QA;

use crate::section_01::ex_01::ex_2_1_1;
use crate::section_01::ex_02::ex_2_1_2;
use crate::section_01::ex_03::ex_2_1_3;
use crate::section_01::ex_04::ex_2_1_4;

mod ex_01;
mod ex_02;
mod ex_03;
mod ex_04;


pub fn main() {
    let exercises = [ex_2_1_1(), ex_2_1_2(), ex_2_1_3(), ex_2_1_4()];
    for e in exercises.iter() { e.run(); }
}