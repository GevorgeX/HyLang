use crate::interpreter::memory::object_utils::{arithmetical::neg, logical::not};
use crate::interpreter::memory::ReferenceToObject;

use super::{Expression, OperationType};

pub struct UnaryExp{
    value: Box<dyn Expression>,
    op: OperationType
}

impl super::Expression for UnaryExp {
    fn evaluate(&self) -> ReferenceToObject {
        match self.op {
            OperationType::Plus => self.value.evaluate(),
            OperationType::Minus => neg(self.value.evaluate()) ,
            OperationType::Not => not(self.value.evaluate()),
            _ => panic!("wrong operator")
        }
    }
}

impl UnaryExp {
    pub fn new(value:Box<dyn Expression>, op:OperationType) -> UnaryExp {
        UnaryExp{value , op}
    }
    // pub fn to_string(&self)->String {
    //     format!("UnaryExp<{}, {}>",self.op.to_string() ,self.value)
    // }
}

