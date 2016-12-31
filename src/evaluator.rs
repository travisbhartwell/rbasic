use lexer;
use token;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;

#[derive(Debug,Clone)]
enum RBasicValue {
    String(String),
    Number(i32),
    Bool(bool),
}

#[derive(Debug)]
struct RBasicContext {
    variables: HashMap<String, RBasicValue>,
}

impl RBasicContext {
    fn new() -> RBasicContext {
        RBasicContext { variables: HashMap::new() }
    }
}

pub fn evaluate(code_lines: Vec<lexer::LineOfCode>) -> Result<String, String> {
    let mut context = RBasicContext::new();
    let mut lineno_to_code = BTreeMap::new();
    let mut line_map = BTreeMap::new();

    for (index, line) in code_lines.iter().enumerate() {
        line_map.insert(&line.line_number, index);
        lineno_to_code.insert(&line.line_number, &line.tokens);
    }

    let line_numbers: Vec<_> = line_map.keys().clone().collect();
    let num_lines = line_numbers.len();
    let mut line_index = 0;
    // TODO: Feels hacky
    let mut line_has_goto = false;

    loop {
        let line_number = line_numbers[line_index];
        let tokens = &lineno_to_code[line_number];
        let mut token_iter = tokens.iter().peekable();

        // println!("Looking at line: {:?}", line_number);
        if !tokens.is_empty() {
            let lexer::TokenAndPos(pos, ref token) = *token_iter.next().unwrap();
            // Set default value
            line_has_goto = false;

            match *token {
                token::Token::Rem => {
                    // Skip the rest of the line so do nothing
                }

                token::Token::Goto => {
                    line_has_goto = true;
                    match token_iter.next() {
                        Some(&lexer::TokenAndPos(pos, token::Token::Number(number))) => {
                            let n = lexer::LineNumber(number as u32);
                            match line_map.get(&n) {
                                Some(index) => line_index = *index,
                                _ => {
                                    return Err(format!("At {:?}, {} invalid target line for GOTO",
                                                       line_number,
                                                       pos))
                                }
                            }
                        }
                        _ => {
                            // TODO: Line # and Column # are 0 based, make readable
                            return Err(format!("At {:?}, {} GOTO must be followed by valid line \
                                                number",
                                               line_number,
                                               pos));
                        }
                    }
                }

                token::Token::Let => {
                    // Expected Next:
                    // Variable Equals EXPRESSION
                    match (token_iter.next(),
                           token_iter.next(),
                           parse_and_eval_expression(&mut token_iter, &context)) {
                        (Some(&lexer::TokenAndPos(_, token::Token::Variable(ref variable))),
                         Some(&lexer::TokenAndPos(_, token::Token::Equals)),
                         Ok(ref value)) => {
                            context.variables
                                .entry(variable.clone().to_string())
                                .or_insert(value.clone());
                        }
                        _ => {
                            return Err(format!("At {:?}, {} invalid syntax for LET.",
                                               line_number,
                                               pos));
                        }

                    }
                }

                token::Token::Print => {
                    // Expected Next:
                    // EXPRESSION
                    match parse_and_eval_expression(&mut token_iter, &context) {
                        Ok(RBasicValue::String(value)) => println!("{}", value),
                        Ok(RBasicValue::Number(value)) => println!("{}", value),
                        Ok(RBasicValue::Bool(value)) => println!("{}", value),
                        Err(_) => {
                            return Err(format!("At {:?}. {} PRINT must be followed by valid \
                                                expression",
                                               line_number,
                                               pos))
                        }
                    }
                }

                token::Token::Input => {
                    match token_iter.next() {
                        Some(&lexer::TokenAndPos(_, token::Token::Variable(ref variable))) => {
                            let mut input = String::new();

                            io::stdin()
                                .read_line(&mut input)
                                .expect("failed to read line");
                            input = input.trim().to_string();
                            let value = RBasicValue::String(input);

                            // Store the string now, can coerce to number later if needed
                            // Can overwrite an existing value
                            context.variables
                                .entry(variable.clone().to_string())
                                .or_insert(value);
                        }

                        _ => {
                            return Err(format!("At {:?}, {} INPUT must be followed by a \
                                                variable name",
                                               line_number,
                                               pos));
                        }
                    }
                }

                token::Token::If => {
                    // Expected Next:
                    // EXPRESSION Then Number
                    // Where Number is a Line Number
                    match (parse_and_eval_expression(&mut token_iter, &context),
                           token_iter.next(),
                           token_iter.next()) {
                        (Ok(RBasicValue::Bool(ref value)),
                         Some(&lexer::TokenAndPos(_, token::Token::Then)),
                         Some(&lexer::TokenAndPos(_, token::Token::Number(ref number)))) => {
                            if *value {
                                line_has_goto = true;
                                let n = lexer::LineNumber(*number as u32);
                                match line_map.get(&n) {
                                    Some(index) => line_index = *index,
                                    _ => {
                                        return Err(format!("At {:?}, {} invalid target line for \
                                                            IF",
                                                           line_number,
                                                           pos))
                                    }
                                }

                            }
                        }
                        _ => {
                            return Err(format!("At {:?}, {}, invalid syntax for IF.",
                                               line_number,
                                               pos));
                        }
                    }
                }

                _ => {
                    return Err(format!("At {:?}, {} invalid syntax", line_number, pos));
                }
            }
        }

        // At end of execution, show context:
        // println!("Current context: {:?}", context);

        if !line_has_goto {
            line_index += 1;
            if line_index == num_lines {
                break;
            }
        }
    }

    Ok("Completed Successfully".to_string())
}

fn parse_expression<'a>(mut token_iter: &mut Peekable<Iter<'a, lexer::TokenAndPos>>)
                        -> Result<VecDeque<token::Token>, String> {
    let mut output_queue: VecDeque<token::Token> = VecDeque::new();
    let mut operator_stack: Vec<token::Token> = Vec::new();

    loop {
        match token_iter.peek() {
            Some(&&lexer::TokenAndPos(_, token::Token::Then)) |
            None => break,
            _ => {}
        }

        match token_iter.next() {
            Some(&lexer::TokenAndPos(_, ref value_token)) if value_token.is_value() => {
                output_queue.push_back(value_token.clone())
            }
            Some(&lexer::TokenAndPos(_, ref op_token)) if op_token.is_operator() => {
                if !operator_stack.is_empty() {
                    let top_op = operator_stack.last().unwrap().clone();
                    if top_op.is_operator() {
                        let associativity = op_token.operator_associavity().unwrap();

                        if (associativity == token::Associativity::Left &&
                            op_token.operator_precedence() <= top_op.operator_precedence()) ||
                           (associativity == token::Associativity::Right &&
                            op_token.operator_precedence() < top_op.operator_precedence()) {
                            let top_op = operator_stack.pop().unwrap();
                            output_queue.push_back(top_op.clone());
                        }
                    }
                }

                operator_stack.push(op_token.clone());
            }
            Some(&lexer::TokenAndPos(_, token::Token::LParen)) => {
                operator_stack.push(token::Token::LParen);
            }
            Some(&lexer::TokenAndPos(_, token::Token::RParen)) => {
                loop {
                    match operator_stack.pop() {
                        Some(token::Token::LParen) => break,
                        Some(ref next_token) => output_queue.push_back(next_token.clone()),
                        None => return Err("Mismatched parenthesis in expression".to_string()),
                    }
                }
            }
            _ => panic!("Shouldn't get here!".to_string()),
        }
    }

    while !operator_stack.is_empty() {
        match operator_stack.pop().unwrap() {
            token::Token::LParen | token::Token::RParen => {
                return Err("Mismatched parenthesis in expression.".to_string())
            }
            op_token => output_queue.push_back(op_token.clone()),
        }
    }

    Ok(output_queue)
}

fn parse_and_eval_expression<'a>(mut token_iter: &mut Peekable<Iter<'a, lexer::TokenAndPos>>,
                                 context: &RBasicContext)
                                 -> Result<RBasicValue, String> {
    match parse_expression(token_iter) {
        Ok(mut output_queue) => {
            let mut stack: Vec<RBasicValue> = Vec::new();

            while !output_queue.is_empty() {
                match output_queue.pop_front() {
                    Some(token::Token::Number(ref number)) => {
                        stack.push(RBasicValue::Number(number.clone()))
                    }
                    Some(token::Token::BString(ref bstring)) => {
                        stack.push(RBasicValue::String(bstring.clone()))
                    }
                    Some(token::Token::Variable(ref name)) => {
                        match context.variables.get(name) {
                            Some(value) => stack.push(value.clone()),
                            None => {
                                return Err(format!("Invalid variable reference {} in expression",
                                                   name))
                            }
                        }
                    }
                    Some(token::Token::UMinus) => {
                        if stack.len() >= 1 {
                            match stack.pop().unwrap() {
                                RBasicValue::Number(ref number) => {
                                    stack.push(RBasicValue::Number(-number.clone()))
                                }
                                _ => return Err("Cannot negate non-numeric values!".to_string()),
                            }
                        } else {
                            return Err("Unary minus requires an operand!".to_string());
                        }
                    }
                    Some(token::Token::Bang) => {
                        if stack.len() >= 1 {
                            match stack.pop().unwrap() {
                                RBasicValue::Bool(ref boolean) => {
                                    stack.push(RBasicValue::Bool(!boolean))
                                }
                                _ => {
                                    return Err("Cannot Boolean not non-Boolean values!".to_string())
                                }
                            }
                        } else {
                            return Err("Boolean not requires an operand!".to_string());
                        }
                    }
                    Some(token::Token::Plus) => {
                        if stack.len() >= 2 {
                            match (stack.pop().unwrap(), stack.pop().unwrap()) {
                                (RBasicValue::Number(operand2), RBasicValue::Number(operand1)) => {
                                    stack.push(RBasicValue::Number(operand1 + operand2))
                                }
                                (RBasicValue::String(operand2), RBasicValue::String(operand1)) => {
                                    stack.push(RBasicValue::String(format!("{}{}",
                                                                           operand1,
                                                                           operand2)));
                                }
                                (RBasicValue::Number(operand2), RBasicValue::String(operand1)) => {
                                    if i32::from_str(operand1.as_str()).is_ok() {
                                        stack.push(RBasicValue::Number(
                                            i32::from_str(operand1.as_str()).unwrap() + operand2));
                                    } else {
                                        return Err(format!("Cannot add integer {} and string {}!",
                                                           operand2,
                                                           operand1));
                                    }
                                }
                                (_, _) => {
                                    return Err("Can only add integers and integers and strings \
                                                together"
                                        .to_string());
                                }
                            }
                        }
                    }
                    Some(token::Token::Minus) => {
                        if stack.len() >= 2 {
                            match (stack.pop().unwrap(), stack.pop().unwrap()) {
                                (RBasicValue::Number(operand2), RBasicValue::Number(operand1)) => {
                                    stack.push(RBasicValue::Number(operand1 - operand2))
                                }
                                (RBasicValue::Number(operand2), RBasicValue::String(operand1)) => {
                                    if i32::from_str(operand1.as_str()).is_ok() {
                                        stack.push(RBasicValue::Number(
                                            i32::from_str(operand1.as_str()).unwrap() - operand2));
                                    } else {
                                        return Err(format!("Cannot subtract integer {} and \
                                                            string {}!",
                                                           operand2,
                                                           operand1));
                                    }
                                }
                                (_, _) => {
                                    return Err("Can only subtract integers".to_string());
                                }
                            }
                        }
                    }
                    Some(token::Token::Multiply) => {
                        if stack.len() >= 2 {
                            match (stack.pop().unwrap(), stack.pop().unwrap()) {
                                (RBasicValue::Number(operand2), RBasicValue::Number(operand1)) => {
                                    stack.push(RBasicValue::Number(operand1 * operand2))
                                }
                                (RBasicValue::Number(operand2), RBasicValue::String(operand1)) => {
                                    if i32::from_str(operand1.as_str()).is_ok() {
                                        stack.push(RBasicValue::Number(
                                            i32::from_str(operand1.as_str()).unwrap() * operand2));
                                    } else {
                                        return Err(format!("Cannot multiply integer {} and \
                                                            string {}!",
                                                           operand2,
                                                           operand1));
                                    }
                                }
                                (_, _) => {
                                    return Err("Can only multiply integers".to_string());
                                }
                            }
                        }
                    }
                    Some(token::Token::Divide) => {
                        if stack.len() >= 2 {
                            match (stack.pop().unwrap(), stack.pop().unwrap()) {
                                (RBasicValue::Number(operand2), RBasicValue::Number(operand1)) => {
                                    stack.push(RBasicValue::Number(operand1 / operand2))
                                }
                                (RBasicValue::Number(operand2), RBasicValue::String(operand1)) => {
                                    if i32::from_str(operand1.as_str()).is_ok() {
                                        stack.push(RBasicValue::Number(
                                            i32::from_str(operand1.as_str()).unwrap() / operand2));
                                    } else {
                                        return Err(format!("Cannot divide integer {} and string \
                                                            {}!",
                                                           operand2,
                                                           operand1));
                                    }
                                }
                                (_, _) => {
                                    return Err("Can only divide integers".to_string());
                                }
                            }
                        }
                    }

                    Some(token::Token::Equals) => {
                        if stack.len() >= 2 {
                            match (stack.pop().unwrap(), stack.pop().unwrap()) {
                                (RBasicValue::Number(operand2), RBasicValue::Number(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 == operand2))
                                }
                                (RBasicValue::String(operand2), RBasicValue::String(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 == operand2))
                                }
                                (RBasicValue::Bool(operand2), RBasicValue::Bool(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 == operand2))
                                }
                                (RBasicValue::Number(operand2), RBasicValue::String(operand1)) => {
                                    if i32::from_str(operand1.as_str()).is_ok() {
                                        stack.push(
                                            RBasicValue::Bool(
                                                i32::from_str(
                                                    operand1.as_str()).unwrap() == operand2));
                                    } else {
                                        return Err(format!("Cannot compare integer {} and \
                                                            string {}!",
                                                           operand2,
                                                           operand1));
                                    }
                                }
                                (_, _) => {
                                    return Err("Can only compare similar types".to_string());
                                }
                            }
                        }
                    }
                    Some(token::Token::NotEqual) => {
                        if stack.len() >= 2 {
                            match (stack.pop().unwrap(), stack.pop().unwrap()) {
                                (RBasicValue::Number(operand2), RBasicValue::Number(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 != operand2))
                                }
                                (RBasicValue::String(operand2), RBasicValue::String(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 != operand2))
                                }
                                (RBasicValue::Bool(operand2), RBasicValue::Bool(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 != operand2))
                                }
                                (RBasicValue::Number(operand2), RBasicValue::String(operand1)) => {
                                    if i32::from_str(operand1.as_str()).is_ok() {
                                        stack.push(
                                            RBasicValue::Bool(
                                                i32::from_str(
                                                    operand1.as_str()).unwrap() != operand2));
                                    } else {
                                        return Err(format!("Cannot compare integer {} and \
                                                            string {}!",
                                                           operand2,
                                                           operand1));
                                    }
                                }
                                (_, _) => {
                                    return Err("Can only compare similar types".to_string());
                                }
                            }
                        }
                    }

                    Some(token::Token::LessThan) => {
                        if stack.len() >= 2 {
                            match (stack.pop().unwrap(), stack.pop().unwrap()) {
                                (RBasicValue::Number(operand2), RBasicValue::Number(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 < operand2))
                                }
                                (RBasicValue::String(operand2), RBasicValue::String(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 < operand2))
                                }
                                (RBasicValue::Number(operand2), RBasicValue::String(operand1)) => {
                                    if i32::from_str(operand1.as_str()).is_ok() {
                                        stack.push(
                                            RBasicValue::Bool(
                                                i32::from_str(
                                                    operand1.as_str()).unwrap() < operand2));
                                    } else {
                                        return Err(format!("Cannot compare integer {} and \
                                                            string {}!",
                                                           operand2,
                                                           operand1));
                                    }
                                }
                                (_, _) => {
                                    return Err("Can only compare similar types".to_string());
                                }
                            }
                        }
                    }
                    Some(token::Token::GreaterThan) => {
                        if stack.len() >= 2 {
                            match (stack.pop().unwrap(), stack.pop().unwrap()) {
                                (RBasicValue::Number(operand2), RBasicValue::Number(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 > operand2))
                                }
                                (RBasicValue::String(operand2), RBasicValue::String(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 > operand2))
                                }
                                (RBasicValue::Number(operand2), RBasicValue::String(operand1)) => {
                                    if i32::from_str(operand1.as_str()).is_ok() {
                                        stack.push(
                                            RBasicValue::Bool(
                                                i32::from_str(
                                                    operand1.as_str()).unwrap() > operand2));
                                    } else {
                                        return Err(format!("Cannot compare integer {} and \
                                                            string {}!",
                                                           operand2,
                                                           operand1));
                                    }
                                }
                                (_, _) => {
                                    return Err("Can only compare similar types".to_string());
                                }
                            }
                        }
                    }
                    Some(token::Token::LessThanEqual) => {
                        if stack.len() >= 2 {
                            match (stack.pop().unwrap(), stack.pop().unwrap()) {
                                (RBasicValue::Number(operand2), RBasicValue::Number(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 <= operand2))
                                }
                                (RBasicValue::String(operand2), RBasicValue::String(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 <= operand2))
                                }
                                (RBasicValue::Number(operand2), RBasicValue::String(operand1)) => {
                                    if i32::from_str(operand1.as_str()).is_ok() {
                                        stack.push(
                                            RBasicValue::Bool(
                                                i32::from_str(
                                                    operand1.as_str()).unwrap() <= operand2));
                                    } else {
                                        return Err(format!("Cannot compare integer {} and \
                                                            string {}!",
                                                           operand2,
                                                           operand1));
                                    }
                                }
                                (_, _) => {
                                    return Err("Can only compare similar types".to_string());
                                }
                            }
                        }
                    }
                    Some(token::Token::GreaterThanEqual) => {
                        if stack.len() >= 2 {
                            match (stack.pop().unwrap(), stack.pop().unwrap()) {
                                (RBasicValue::Number(operand2), RBasicValue::Number(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 >= operand2))
                                }
                                (RBasicValue::String(operand2), RBasicValue::String(operand1)) => {
                                    stack.push(RBasicValue::Bool(operand1 >= operand2))
                                }
                                (RBasicValue::Number(operand2), RBasicValue::String(operand1)) => {
                                    if i32::from_str(operand1.as_str()).is_ok() {
                                        stack.push(
                                            RBasicValue::Bool(
                                                i32::from_str(
                                                    operand1.as_str()).unwrap() >= operand2));
                                    } else {
                                        return Err(format!("Cannot compare integer {} and \
                                                            string {}!",
                                                           operand2,
                                                           operand1));
                                    }
                                }
                                (_, _) => {
                                    return Err("Can only compare similar types".to_string());
                                }
                            }
                        }
                    }
                    None => panic!("Shouldn't reach this, None!"),
                    _ => panic!("Shouldn't reach this!"),
                }
            }

            // If expression is well formed, there will only be the result on the stack
            assert!(stack.len() == 1);
            Ok(stack[0].clone())
        }

        _ => return Err("Invalid expression!".to_string()),
    }
}
