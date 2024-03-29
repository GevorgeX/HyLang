use crate::interpreter::memory::object_utils::{conditional, logical};
use crate::interpreter::memory::ReferenceToObject;

use super::{Expression, OperationType};

pub struct ConditionalExp{
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    op: OperationType
}

impl super::Expression for ConditionalExp {
    fn evaluate(&self) -> ReferenceToObject {
        match self.op {
            OperationType::And => logical::and(self.left.evaluate(), self.right.evaluate())  ,
            OperationType::Or =>logical::or(self.left.evaluate(), self.right.evaluate())  ,
            OperationType::More =>conditional::more(self.left.evaluate(), self.right.evaluate())  ,
            OperationType::Less =>conditional::less(self.left.evaluate(), self.right.evaluate())  ,
            OperationType::MoreOrEq =>conditional::more_or_eq(self.left.evaluate(), self.right.evaluate())  ,
            OperationType::LessOrEq =>conditional::less_or_eq(self.left.evaluate(), self.right.evaluate())  ,
            OperationType::DoubleEqual => conditional::equal(self.left.evaluate(), self.right.evaluate())  ,
            OperationType::NotEqual => conditional::not_equal(self.left.evaluate(), self.right.evaluate())  ,
            _ => panic!("Cant use this operator")
        }
    }
}

impl ConditionalExp {
    pub fn new(left:Box<dyn Expression> , right:Box<dyn Expression>, op:OperationType) -> ConditionalExp {
        ConditionalExp{left, right , op}
    }
}

