use crate::interpreter::{expression::Expression, task::Task, MemRef};

pub struct DefineVariableStm{
    name: String,
    value: Box<dyn Expression>,
    mem: MemRef
}

impl super::Statement for DefineVariableStm {
    fn interpret(&self) -> Task {
        self.mem.define_variable(self.name.clone(), self.value.evaluate());

        Task::DefineReference(self.name.clone())
    }
}

impl DefineVariableStm {
    pub fn new( name: String,value :Box<dyn Expression> , mem:MemRef) -> DefineVariableStm {
        DefineVariableStm{name , value , mem}
    }
}