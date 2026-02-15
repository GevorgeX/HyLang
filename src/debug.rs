use crate::parser::expression::Expression;
use crate::parser::statement::Statement;
use crate::parser::declaration::Declaration;
use crate::parser::types::Type;

fn make_prefix(indent: usize, is_last: bool) -> String {
    if indent == 0 {
        String::new()
    } else {
        let mut s = String::new();
        for _ in 0..(indent - 1) {
            s.push_str("│   ");
        }
        s.push_str(if is_last { "└── " } else { "├── " });
        s
    }
}

pub fn print_expression_tree(expr: &Expression, indent: usize, is_last: bool, code: &String) {
    let prefix = make_prefix(indent, is_last);

    match expr {
        Expression::Integer{token_info} => {
            println!("{}Integer {}", prefix, &code[token_info.index..token_info.index + token_info.len]);
        }
        Expression::Boolean{token_info} => {
            println!("{}Boolean {}", prefix, &code[token_info.index..token_info.index + token_info.len]);

        }
        Expression::Identifier{token_info} => {
            println!("{}Identifier {}", prefix, &code[token_info.index..token_info.index + token_info.len]);

        }
        Expression::Unary{operator, right, token_info} => {
            println!("{}Unary {:?} {}", prefix, operator, &code[token_info.index..token_info.index + token_info.len]);

            print_expression_tree(right, indent + 1, true, code);
        }
        Expression::Binary{left, operator, right, ..} => {
            println!("{}Binary {:?}", prefix, operator);

            print_expression_tree(left, indent + 1, false, code);
            print_expression_tree(right, indent + 1, true, code);
        }
        Expression::GetMember{object, member, ..} => {
            println!("{}GetMember", prefix);
            print_expression_tree(object, indent + 1, false, code);
            print_expression_tree(member, indent + 1, true, code);
        }
        Expression::Call{callee, arguments, ..} => {
            println!("{}Call", prefix);
            print_expression_tree(callee, indent + 1, false, code);
            for (i, arg) in arguments.iter().enumerate() {
                print_expression_tree(arg, indent + 1, i == arguments.len() - 1, code);
            }
        }
        Expression::Index{object, index, ..} => {
            println!("{}Index", prefix);
            print_expression_tree(object, indent + 1, false, code);
            print_expression_tree(index, indent + 1, true, code);
        }
        Expression::While{condition, body, ..} => {
            println!("{}While", prefix);
            print_expression_tree(condition, indent + 1, false, code);
            for (i, stmt) in body.statements.iter().enumerate() {
                print_statement_tree(stmt, indent + 1, i == body.statements.len() - 1, code);
            }
        }
        Expression::Block{body} => {
            println!("{}Block", prefix);
            for (i, stmt) in body.statements.iter().enumerate() {
                print_statement_tree(stmt, indent + 1, i == body.statements.len() - 1, code);
            }
        }
        Expression::IfElse{if_block, else_if_blocks, else_block} => {
            println!("{}If", prefix);
            // Печать условия if
            print_expression_tree(&if_block.condition, indent + 1, true, code);

            for (i, stmt) in if_block.body.statements.iter().enumerate() {
                print_statement_tree(stmt, indent + 1, i == if_block.body.statements.len() - 1, code);
            }
            // Печать elif ветвей
            if let Some(elif_blocks) = else_if_blocks {
                for elif in elif_blocks {
                    let elif_prefix = make_prefix(indent, false);
                    println!("{}Elif", elif_prefix);
                    print_expression_tree(&elif.condition, indent + 1, true, code);
                    for (i, stmt) in elif.body.statements.iter().enumerate() {
                        print_statement_tree(stmt, indent + 1, i == elif.body.statements.len() - 1, code);
                    }
                }
            }
            // Печать else ветви
            if let Some(else_block) = else_block {
                let else_prefix = make_prefix(indent, true);
                println!("{}Else", else_prefix);
                for (i, stmt) in else_block.body.statements.iter().enumerate() {
                    print_statement_tree(stmt, indent + 1, i == else_block.body.statements.len() - 1, code);
                }
            }
        }
    }
}

pub fn print_statement_tree(stmt: &Statement, indent: usize, is_last: bool, code: &String) {
    let prefix = make_prefix(indent, is_last);

    match stmt {
        Statement::ExpressionStatement(expr) => {
            println!("{}ExpressionStatement", prefix);
            print_expression_tree(expr, indent + 1, true, code);
        }
        Statement::DefineVariable{identifier, value, var_type} => {
            println!("{}DefineVariable {:?}", prefix, &code[identifier.index..identifier.index + identifier.len]);
            if let Some(ty) = var_type {
                print_type_tree(ty, indent + 1, true, code);
            }
            if let Some((_, expr)) = value {
                print_expression_tree(expr, indent + 1, true, code);
            }
        }
        Statement::DefineConstantVariable{identifier, value, const_type} => {
            println!("{}DefineConstantVariable {:?}", prefix, &code[identifier.index..identifier.index + identifier.len]);
            if let Some(ty) = const_type {
                print_type_tree(ty, indent + 1, true, code);
            }
            if let Some((_, expr)) = value {
                print_expression_tree(expr, indent + 1, true, code);
            }
        }
    }
}

fn print_fields(fields: &[crate::parser::declaration::struct_union_define::Field], indent: usize, code: &String) {
    for (i, field) in fields.iter().enumerate() {
        let field_prefix = format!(
            "{}{}",
            "│   ".repeat(indent),
            if i == fields.len() - 1 { "└───" } else { "├───" }
        );
        let name = field.name_token_info;
        println!(
            "{}Field name={:?}",
            field_prefix,
            &code[name.index..name.index + name.len],
        );
        print_type_tree(&field.field_type, indent + 2, true, code);
    }
}

pub fn print_declaration_tree(decl: &Declaration, indent: usize, is_last: bool, code: &String) {
    let prefix = make_prefix(indent, is_last);

    match decl {
        Declaration::Namespace { name, body, .. } => {
            println!("{}Namespace {:?}", prefix, &code[name.index..name.index + name.len]);

            for (i, sub_decl) in body.iter().enumerate() {
                print_declaration_tree(sub_decl, indent + 1, i == body.len() - 1, code);
            }
        }
        Declaration::Function{ name, parameters, return_type, body, .. } => {
            println!(
                "{}FunctionDefine name={:?}",
                prefix, &code[name.index..name.index + name.len]
            );

            for (i, field) in parameters.iter().enumerate() {
                let field_prefix = format!(
                    "{}{}",
                    "│   ".repeat(indent),
                    if i == parameters.len() - 1
                        && return_type.is_none()
                        && body.statements.len() == 0 { "└───" } else { "├───" }
                );
                let name = field.name_token_info;
                println!(
                    "{}Argument name={:?}",
                    field_prefix,
                    &code[name.index..name.index + name.len],
                );
                print_type_tree(&field.arg_type, indent + 2, true, code);
            }

            if let Some(ret_type) = return_type {
                println!("{}├───ReturnType", "│   ".repeat(indent));
                print_type_tree(ret_type, indent + 2, true, code);
            }

            for (i, stmt) in body.statements.iter().enumerate() {
                print_statement_tree(stmt, indent + 1, i == body.statements.len() - 1, code);
            }
        }
        Declaration::Struct { name, fields, .. } => {
            println!(
                "{}Struct name={:?} ",
                prefix, &code[name.index..name.index + name.len]
            );
            print_fields(fields, indent, code);
        }
        Declaration::Union { name, fields, .. } => {
            println!(
                "{}Union name={:?} ",
                prefix, &code[name.index..name.index + name.len]
            );
            print_fields(fields, indent, code);
        }
    }
}

pub fn print_type_tree(ty: &Type, indent: usize, is_last: bool, code: &str) {
    let prefix = make_prefix(indent, is_last);
    match ty {
        Type::Identifier { token_info } => {
            let name = &code[token_info.index..token_info.index + token_info.len];
            println!("{}Identifier {}", prefix, name);
        }
        Type::Pointer { to, .. } => {
            println!("{}Pointer", prefix);
            print_type_tree(to, indent + 1, true, code);
        }
    }
}
