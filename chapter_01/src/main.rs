mod section_01;
mod section_02;

fn main() {
    section_01::main();
    section_02::main();
}

const TERMINAL_SIZE: usize = 140;

trait QA {
    fn number(&self) -> String;
    fn question(&self) -> String;
    fn answer(&self) -> String;
    fn run(&self) {
        println!();
        println!("[{}]\n", &self.number());
        println!("{}", &self.question());
        println!();
        println!("{}", &self.answer());
        println!();
        println!("{}", "*".repeat(TERMINAL_SIZE));
    }
}

pub struct Exercise {
    number: String,
    question: String,
    answer: String,
}

impl QA for Exercise {
    fn number(&self) -> String { return String::from(&self.number); }
    fn question(&self) -> String { return String::from(&self.question); }
    fn answer(&self) -> String { return String::from(&self.answer); }
}
