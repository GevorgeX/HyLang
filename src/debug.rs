use crate::parser::expression::{Expression, IfElseBranch, ElseBranch};
use crate::parser::statement::Statement;

pub fn print_expression_tree(expr: &Expression, indent: usize, is_last: bool, show_token_info: bool) {
    let prefix = if indent == 0 {
        String::new()
    } else {
        let mut s = String::new();
        for _ in 0..(indent - 1) {
            s.push_str("│   ");
        }
        s.push_str(if is_last { "└── " } else { "├── " });
        s
    };

    match expr {
        Expression::Integer{token_info} => {
            if show_token_info {
                println!("{}Integer {:?}", prefix, token_info);
            } else {
                println!("{}Integer", prefix);
            }
        }
        Expression::Boolean{token_info} => {
            if show_token_info {
                println!("{}Boolean {:?}", prefix, token_info);
            } else {
                println!("{}Boolean", prefix);
            }
        }
        Expression::Identifier{token_info} => {
            if show_token_info {
                println!("{}Identifier {:?}", prefix, token_info);
            } else {
                println!("{}Identifier", prefix);
            }
        }
        Expression::Unary{operator, right, token_info} => {
            if show_token_info {
                println!("{}Unary {:?} {:?}", prefix, operator, token_info);
            } else {
                println!("{}Unary {:?}", prefix, operator);
            }
            print_expression_tree(right, indent + 1, true, show_token_info);
        }
        Expression::Binary{left, operator, right, token_info} => {
            if show_token_info {
                println!("{}Binary {:?} {:?}", prefix, operator, token_info);
            } else {
                println!("{}Binary {:?}", prefix, operator);
            }
            print_expression_tree(left, indent + 1, false, show_token_info);
            print_expression_tree(right, indent + 1, true, show_token_info);
        }
        Expression::GetMember{object, member, token_info} => {
            if show_token_info {
                println!("{}GetMember {:?}", prefix, token_info);
            } else {
                println!("{}GetMember", prefix);
            }
            print_expression_tree(object, indent + 1, false, show_token_info);
            print_expression_tree(member, indent + 1, true, show_token_info);
        }
        Expression::Call{callee, arguments, token_info} => {
            if show_token_info {
                println!("{}Call {:?}", prefix, token_info);
            } else {
                println!("{}Call", prefix);
            }
            print_expression_tree(callee, indent + 1, false, show_token_info);
            for (i, arg) in arguments.iter().enumerate() {
                print_expression_tree(arg, indent + 1, i == arguments.len() - 1, show_token_info);
            }
        }
        Expression::Index{object, index, token_info} => {
            if show_token_info {
                println!("{}Index {:?}", prefix, token_info);
            } else {
                println!("{}Index", prefix);
            }
            print_expression_tree(object, indent + 1, false, show_token_info);
            print_expression_tree(index, indent + 1, true, show_token_info);
        }
        Expression::While{condition, body, token_info, brackets_token_info} => {
            if show_token_info {
                println!("{}While {:?} {:?}", prefix, token_info, brackets_token_info);
            } else {
                println!("{}While", prefix);
            }
            print_expression_tree(condition, indent + 1, false, show_token_info);
            println!("{}Body", "│   ".repeat(indent));
            for (i, stmt) in body.iter().enumerate() {
                print_statement_tree(stmt, indent + 2, i == body.len() - 1, show_token_info);
            }
        }
        Expression::Block{body, brackets_token_info} => {
            if show_token_info {
                println!("{}Block {:?}", prefix, brackets_token_info);
            } else {
                println!("{}Block", prefix);
            }
            for (i, stmt) in body.iter().enumerate() {
                print_statement_tree(stmt, indent + 1, i == body.len() - 1, show_token_info);
            }
        }
        Expression::IfElse{if_block, else_if_blocks, else_block} => {
            println!("{}IfElse", prefix);
            print_if_else_branch("If", if_block, indent + 1, false, show_token_info);

            if let Some(else_if_blocks) = else_if_blocks {
                for (i, elif) in else_if_blocks.iter().enumerate() {
                    print_if_else_branch("Elif", elif, indent + 1, false, show_token_info);
                }
            }
            if let Some(else_block) = else_block {
                print_else_branch(else_block, indent + 1, true, show_token_info);
            }
        }
    }
}

pub fn print_statement_tree(stmt: &Statement, indent: usize, is_last: bool, show_token_info: bool) {
    let prefix = if indent == 0 {
        String::new()
    } else {
        let mut s = String::new();
        for _ in 0..(indent - 1) {
            s.push_str("│   ");
        }
        s.push_str(if is_last { "└── " } else { "├── " });
        s
    };

    match stmt {
        Statement::ExpressionStatement(expr) => {
            println!("{}ExpressionStatement", prefix);
            print_expression_tree(expr, indent + 1, true, show_token_info);
        }
        Statement::DefineVariable{identifier, value, token_info} => {
            if show_token_info {
                println!("{}DefineVariable {:?} {:?}", prefix, identifier.token_info, token_info);
            } else {
                println!("{}DefineVariable", prefix);
            }
            if let Some((_, expr)) = value {
                print_expression_tree(expr, indent + 1, true, show_token_info);
            }
        }
    }
}

fn print_if_else_branch(label: &str, branch: &IfElseBranch, indent: usize, is_last: bool, show_token_info: bool) {
    let prefix = if indent == 0 {
        String::new()
    } else {
        let mut s = String::new();
        for _ in 0..(indent - 1) {
            s.push_str("│   ");
        }
        s.push_str(if is_last { "└── " } else { "├── " });
        s
    };
    if show_token_info {
        println!("{}{} [{:?} {:?}]", prefix, label, branch.token_info, branch.brackets_token_info);
    } else {
        println!("{}{}", prefix, label);
    }
    print_expression_tree(&branch.condition, indent + 1, false, show_token_info);
    println!("{}Body", "│   ".repeat(indent));
    for (i, stmt) in branch.body.iter().enumerate() {
        print_statement_tree(stmt, indent + 2, i == branch.body.len() - 1, show_token_info);
    }
}

fn print_else_branch(branch: &ElseBranch, indent: usize, is_last: bool, show_token_info: bool) {
    let prefix = if indent == 0 {
        String::new()
    } else {
        let mut s = String::new();
        for _ in 0..(indent - 1) {
            s.push_str("│   ");
        }
        s.push_str(if is_last { "└── " } else { "├── " });
        s
    };
    if show_token_info {
        println!("{}Else [{:?} {:?}]", prefix, branch.token_info, branch.brackets_token_info);
    } else {
        println!("{}Else", prefix);
    }
    println!("{}Body", "│   ".repeat(indent));
    for (i, stmt) in branch.body.iter().enumerate() {
        print_statement_tree(stmt, indent + 2, i == branch.body.len() - 1, show_token_info);
    }
}
