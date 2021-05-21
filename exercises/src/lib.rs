const TERMINAL_SIZE: usize = 140;

pub trait QA {
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
    pub number: String,
    pub question: String,
    pub answer: String,
}

impl QA for Exercise {
    fn number(&self) -> String { return String::from(&self.number); }
    fn question(&self) -> String { return String::from(&self.question); }
    fn answer(&self) -> String { return String::from(&self.answer); }
}
