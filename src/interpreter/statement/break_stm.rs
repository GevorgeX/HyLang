use crate::interpreter::{library::Context, task::Task};

use super::Statement;

pub struct BreakStm{

}

impl Statement for BreakStm{
    fn interpret(&self, context:&Context) -> Task {
        Task::Break
    }
}

impl BreakStm {
    pub fn new()->BreakStm {
        BreakStm{}
    }
}