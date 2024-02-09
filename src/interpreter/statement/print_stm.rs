use crate::interpreter::{expression::Expression, task::Task};

pub struct PrintStm{
    value: Box<dyn Expression>,
    
}

impl super::Statement for PrintStm {
    fn interpret(&self) -> Task{
        println!("{}", self.value.evaluate().to_string());

        Task::Default
    }
}

impl PrintStm {
    pub fn new(value :Box<dyn Expression> ) -> PrintStm {
        PrintStm{ value }
    }
}