use super::{expression::Expression, statement::Statement};

pub trait Node {
    fn get_child(&self , index:usize) -> &dyn Node;
    fn size(&self) -> usize; 
    fn to_string(&self) -> String;
}
pub fn print_tree(tree:&dyn Node, mut indent:String, last:bool){
    println!("{}", indent.clone() + "+- " + &tree.to_string() );
    indent.push_str(if last  {"   "} else {"|  "}) ;

    for i in 0..tree.size()
    {
        print_tree(tree.get_child(i), indent.clone(), i == tree.size() - 1);
    }
}

impl Node for Expression {
    fn get_child(&self , index:usize) -> &dyn Node {
        match self {
            Expression::ArrayExp(_) => match index {
                0 => todo!(),
                _=> todo!()
            } ,
            Expression::BinaryExp(o) => match index {
                0 => &*o.left,
                1 => &*o.right,
                _=> todo!()
            } ,
            Expression::BooleanExp(_) => match index {
                _=> todo!()
            } ,
            Expression::CharExp(_) => match index {
                _=> todo!()
            } ,
            Expression::ConditionalExp(o) => match index {
                0 => &*o.left,
                1 => &*o.right,
                _=> todo!()
            } ,
            Expression::FunctionCallExp(_) => match index {
                0 => todo!(),
                _=> todo!()
            } ,
            Expression::NumberExp(_) => match index {
                _=> todo!()
            } ,
            Expression::UnaryExp(_) => match index {
                _=> todo!()
            } ,
            Expression::ValueExp(_) => match index {
                _=> todo!()
            } ,
        }    }

    fn size(&self) -> usize {
        match self {
            Expression::ArrayExp(_) => 0 ,
            Expression::BinaryExp(_) => 2,
            Expression::BooleanExp(_) => 0,
            Expression::CharExp(_) => 0,
            Expression::ConditionalExp(_) => 2,
            Expression::FunctionCallExp(o) => o.arguments.len(),
            Expression::NumberExp(_) => 0,
            Expression::UnaryExp(_) =>0,
            Expression::ValueExp(_) =>0,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Expression::ArrayExp(_) => "ArrayExp".to_string() ,
            Expression::BinaryExp(o) => format!("BinaryExp op {}" ,o.op.to_string()),
            Expression::BooleanExp(o) => format!("BooleanExp {}",o.value),
            Expression::CharExp(o) => format!("CharExp '{}'",o.value),
            Expression::ConditionalExp(o) => format!("ConditionalExp op {}" ,o.op.to_string()),
            Expression::FunctionCallExp(o) =>  format!("FunctionCallExp '{}'",o.name),
            Expression::NumberExp(o) => format!("NumberExp {}",o.value),
            Expression::UnaryExp(o) =>format!("UnaryExp {}",o.op.to_string()),
            Expression::ValueExp(o) => format!("ValueExp {}",o.name),
        }
    }
}


impl Node for Statement {
    fn get_child(&self , index:usize) -> &dyn Node {
        match self {
            Statement::AssignmentStm(o) => match index {
                0 => &*o.value,
                _ => todo!()
            },
            // Statement::BlockStm(_) => todo!(),
            Statement::BlockStm(o) => {
                &*o.statements[index]
            },
            Statement::BreakStm(_) => match index {
                _ => todo!()
            },
            Statement::ContinueStm(_) => match index {
                _ => todo!()
            },
            Statement::DefineFunctionStm(o) => match index {
                0 =>&*o.body,
                _ => todo!()
            },
            Statement::DefineVariableStm(o) => match index {
                0 => &*o.value,
                _ => todo!()
            },
            Statement::FunctionCallStm(o) => &*o.arguments[index],
            Statement::IfElseStm(o) => match index {
                0 => &*o.condition,
                1 => &*o.if_statement,
                2 => match &o.else_statement {
                    Some(o) => &**o,
                    None => todo!(),
                },
                _ => todo!()
            },
            Statement::PRINTSTM(o) => match index {
                0 => &*o.value,
                _ => todo!()
            },
            Statement::ReturnStm(o) => match index {
                0 => &*o.value,
                _ => todo!()
            },
            Statement::WhileStm(o) => match index {
                0 => &*o.condition,
                1 => &*o.while_statement,
                _ => todo!()
            },
            Statement::EmptyStm(o) => &*o.body,
        }  
    }

    fn size(&self) -> usize {
        match self {
            Statement::AssignmentStm(_) => 1,
            Statement::BlockStm(o) =>  o.statements.len(),
            Statement::BreakStm(_) =>  0,
            Statement::ContinueStm(_) =>  0,
            Statement::DefineFunctionStm(_) =>  1,
            Statement::DefineVariableStm(_) =>  1,
            Statement::FunctionCallStm(_) =>  0,
            Statement::IfElseStm(o) =>  match o.else_statement {
                Some(_) => 3,
                None => 2,
            }
            Statement::PRINTSTM(_) =>  1,
            Statement::ReturnStm(_) =>  1,
            Statement::WhileStm(_) => 2,
            Statement::EmptyStm(_) => 1,
        }    
    }

    fn to_string(&self) -> String {
        match self {
            Statement::AssignmentStm(_) => "AssignmentStm".to_string(),
            Statement::BlockStm(_) =>  "BlockStm".to_string(),
            Statement::BreakStm(_) =>  "BreakStm".to_string(),
            Statement::ContinueStm(_) =>  "ContinueStm".to_string(),
            Statement::DefineFunctionStm(o) =>  format!("DefineFunctionStm {} ({})",o.name ,o.args.join(",")),
            Statement::DefineVariableStm(o) =>  format!("DefineVariableStm {}",o.name),
            Statement::FunctionCallStm(_) =>  "FunctionCallStm".to_string(),
            Statement::IfElseStm(_) =>  "IfElseStm".to_string(),
            Statement::PRINTSTM(_) =>  "PRINTSTM".to_string(),
            Statement::ReturnStm(_) =>  "ReturnStm".to_string(),
            Statement::WhileStm(_) => "WhileStm".to_string(),
            Statement::EmptyStm(_) => "EmptyStm".to_string(),
        }
    }
}