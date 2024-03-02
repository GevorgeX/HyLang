use std::rc::Rc;

use crate::interpreter::{library::Context, task::Task};

use super::Statement;

#[derive(Clone)]
pub struct BreakStm{

}

impl Statement for BreakStm{
    fn interpret(&self, _context:Rc<Context>) -> Task {
        Task::Break
    }
}

impl BreakStm {
    pub fn new()->BreakStm {
        BreakStm{}
    }
}