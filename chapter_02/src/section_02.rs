use exercises::QA;

use crate::section_02::ex_01::ex_2_2_1;
use crate::section_02::ex_02::ex_2_2_2;
use crate::section_02::ex_03::ex_2_2_3;
use crate::section_02::ex_04::ex_2_2_4;

mod ex_01;
mod ex_02;
mod ex_03;
mod ex_04;

pub fn main() {
    let exercises = [ex_2_2_1(), ex_2_2_2(), ex_2_2_3(), ex_2_2_4()];
    for e in exercises.iter() { e.run(); }
}