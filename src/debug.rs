use crate::parser::expression::{Expression, UnaryOperator, BinaryOperator};

pub fn print_expression_tree(expr: &Expression, indent: usize) {
    let padding = if indent == 0 {
        String::new()
    } else {
        format!("{}{}", "    ".repeat((indent - 1) / 4), "└── ")
    };
    match expr {
        Expression::Integer { token_info } => {
            println!("{}Integer: {:?}", padding, token_info);
        }
        Expression::Boolean { token_info } => {
            println!("{}Boolean: {:?}", padding, token_info);
        }
        Expression::Identifier { token_info } => {
            println!("{}Identifier: {:?}", padding, token_info);
        }
        Expression::Unary { operator, right, token_info } => {
            println!("{}Unary: {:?} {:?}", padding, operator, token_info);
            print_expression_tree(right, indent + 4);
        }
        Expression::Binary { left, operator, right, token_info } => {
            println!("{}Binary: {:?} {:?}", padding, operator, token_info);
            print_expression_tree(left, indent + 4);
            print_expression_tree(right, indent + 4);
        }
    }
}