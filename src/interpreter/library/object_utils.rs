pub mod logical {
    use std::rc::Rc;

    use crate::interpreter::library::object::Object;
    use crate::interpreter::library::ReferenceToObject;

    pub fn and(left: ReferenceToObject, right: ReferenceToObject) -> ReferenceToObject {
        match (*left , *right) {
            (Object::Bool(v1), Object::Bool(v2)) => Rc::new(Object::Bool(v1 && v2)),
            _ => panic!("Cant use 'and' with {} and {}", left.to_string(), right.to_string())
        }
    }

    pub fn or(left: ReferenceToObject, right: ReferenceToObject) -> ReferenceToObject {
        match (*left , *right) {
            (Object::Bool(v1), Object::Bool(v2)) => Rc::new(Object::Bool(v1 || v2)),
            _ => panic!("Cant use 'or' with {} and {}", left.to_string(), right.to_string())
        }    
    }

    pub fn not(left: ReferenceToObject) -> ReferenceToObject {
        match *left {
            Object::Bool(v1) => Rc::new(Object::Bool(!v1)),
            _ => panic!("Cant use '!' with {} ", left.to_string())
        }    
    }
}

pub mod arithmetical {
    use std::rc::Rc;

    use crate::interpreter::library::object::Object;
    use crate::interpreter::library::ReferenceToObject;

    pub fn add(left: ReferenceToObject, right: ReferenceToObject) -> ReferenceToObject {
        match (*left , *right) {
            (Object::Number(v1), Object::Number(v2)) => Rc::new(Object::Number(v1 + v2)),
            _ => panic!("Cant use '+' with {} and {}", left.to_string(), right.to_string())
        }  
    }

    pub fn sub(left: ReferenceToObject, right: ReferenceToObject) -> ReferenceToObject {
        match (*left , *right) {
            (Object::Number(v1), Object::Number(v2)) => Rc::new(Object::Number(v1 - v2)),
            _ => panic!("Cant use '-' with {} and {}", left.to_string(), right.to_string())
        }    
     }

    pub fn mult(left: ReferenceToObject, right: ReferenceToObject) -> ReferenceToObject {
        match (*left , *right) {
            (Object::Number(v1), Object::Number(v2)) => Rc::new(Object::Number(v1 * v2)),
            _ => panic!("Cant use '*' with {} and {}", left.to_string(), right.to_string())
        }     
    }

    pub fn div(left: ReferenceToObject, right: ReferenceToObject) -> ReferenceToObject {
        match (*left , *right) {
            (Object::Number(v1), Object::Number(v2)) => Rc::new(Object::Number(v1 / v2)),
            _ => panic!("Cant use '/' with {} and {}", left.to_string(), right.to_string())
        }   
    }

    pub fn neg(left: ReferenceToObject) -> ReferenceToObject {
        match *left  {
            Object::Number(v1) => Rc::new(Object::Number(-v1)),
            _ => panic!("Cant use '-' with {} ", left.to_string())
        } 
    }
}

pub mod conditional {
    use std::rc::Rc;

    use crate::interpreter::library::object::Object;
    use crate::interpreter::library::ReferenceToObject;

    pub fn more(left: ReferenceToObject, right: ReferenceToObject) -> ReferenceToObject {
        match (*left , *right) {
            (Object::Number(v1), Object::Number(v2)) => Rc::new(Object::Bool(v1 > v2)),
            _ => panic!("Cant use '>' with {} and {}", left.to_string(), right.to_string())
        } 
    }

    pub fn less(left: ReferenceToObject, right: ReferenceToObject) -> ReferenceToObject {
        match (*left , *right) {
            (Object::Number(v1), Object::Number(v2)) => Rc::new(Object::Bool(v1 < v2)),
            _ => panic!("Cant use '<' with {} and {}", left.to_string(), right.to_string())
        } 
    }

    pub fn more_or_eq(left: ReferenceToObject, right: ReferenceToObject) -> ReferenceToObject {
        match (*left , *right) {
            (Object::Number(v1), Object::Number(v2)) => Rc::new(Object::Bool(v1 >= v2)),
            _ => panic!("Cant use '>=' with {} and {}", left.to_string(), right.to_string())
        } 
    }

    pub fn less_or_eq(left: ReferenceToObject, right: ReferenceToObject) -> ReferenceToObject {
        match (*left , *right) {
            (Object::Number(v1), Object::Number(v2)) => Rc::new(Object::Bool(v1 <= v2)),
            _ => panic!("Cant use '<=' with {} and {}", left.to_string(), right.to_string())
        } 
    }

    pub fn equal(left: ReferenceToObject, right: ReferenceToObject) -> ReferenceToObject {
        match (*left , *right) {
            (Object::Number(v1), Object::Number(v2)) => Rc::new(Object::Bool(v1 == v2)),
            (Object::Char(v1), Object::Char(v2)) => Rc::new(Object::Bool(v1 == v2)),
            (Object::Bool(v1), Object::Bool(v2)) => Rc::new(Object::Bool(v1 == v2)),

            _ => panic!("Cant use '==' with {} and {}", left.to_string(), right.to_string())
        } 
    }

    pub fn not_equal(left: ReferenceToObject, right: ReferenceToObject) -> ReferenceToObject {
        match (*left , *right) {
            (Object::Number(v1), Object::Number(v2)) => Rc::new(Object::Bool(v1 != v2)),
            (Object::Char(v1), Object::Char(v2)) => Rc::new(Object::Bool(v1 != v2)),
            (Object::Bool(v1), Object::Bool(v2)) => Rc::new(Object::Bool(v1 != v2)),

            _ => panic!("Cant use '!=' with {} and {}", left.to_string(), right.to_string())
        } 
    }
}