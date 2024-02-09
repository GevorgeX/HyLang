use crate::interpreter::memory::object_utils::arithmetical;
use crate::interpreter::memory::ReferenceToObject;

use super::{Expression, OperationType};

pub struct BinaryExp{
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    op: OperationType
}

impl super::Expression for BinaryExp {
    fn evaluate(&self) -> ReferenceToObject {
        match self.op {
            OperationType::Plus => arithmetical::add(self.left.evaluate(), self.right.evaluate()) ,
            OperationType::Minus => arithmetical::sub(self.left.evaluate(), self.right.evaluate()) ,
            OperationType::Multi => arithmetical::mult(self.left.evaluate(), self.right.evaluate()) ,
            OperationType::Divide => arithmetical::div(self.left.evaluate(), self.right.evaluate()) ,
            _ => panic!("Cant use this operator")
        }
    }
}

impl BinaryExp {
    pub fn new(left:Box<dyn Expression> , right:Box<dyn Expression>, op:OperationType) -> BinaryExp {
        BinaryExp{left, right , op}
    }
}

