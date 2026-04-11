use crate::lexer;
use crate::lexer::LineNumber;
use crate::token;
use crate::value;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;

use rand::RngExt;

#[derive(Debug)]
struct RBasicContext {
    variables: HashMap<String, value::RBasicValue>,
}

impl RBasicContext {
    fn new() -> RBasicContext {
        RBasicContext {
            variables: HashMap::new(),
        }
    }
}

pub fn get_line_map<'a>(
    code_lines: &'a [lexer::LineOfCode],
    btree: &mut BTreeMap<LineNumber, &'a lexer::LineOfCode>,
) {
    for line in code_lines {
        btree.insert(line.line_number.clone(), line);
    }
}

pub fn evaluate(code_lines: Vec<lexer::LineOfCode>) -> Result<String, String> {
    let mut context = RBasicContext::new();
    let mut line_map = BTreeMap::new();

    get_line_map(&code_lines, &mut line_map);

    // TODO: Feels hacky
    let mut line_has_goto = false;

    // Start from the first line in the BTreeMap
    let mut current_line_number = match line_map.keys().next() {
        Some(n) => *n,
        None => return Ok("Completed Successfully".to_string()),
    };

    loop {
        let line = match line_map.get(&current_line_number) {
            Some(l) => l,
            None => break,
        };
        let tokens = &line.tokens;
        let mut token_iter = tokens.iter().peekable();

        // println!("Looking at line: {:?}", current_line_number);
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
                            let n = number as u32;
                            match line_map.get(&n) {
                                Some(_target_line) => current_line_number = n,
                                _ => {
                                    return Err(format!(
                                        "At {:?}, {} invalid target line for GOTO",
                                        line.line_number, pos
                                    ))
                                }
                            }
                        }
                        Some(&lexer::TokenAndPos(pos, _)) => {
                            return Err(format!(
                                "At {:?}, {} GOTO must be followed by valid line \
                                                number",
                                line.line_number, pos
                            ));
                        }
                        None => {
                            return Err(format!(
                                "At {:?}, {} GOTO must be followed by a line \
                                                number",
                                line.line_number,
                                // Adding 4 to give the position past GOTO
                                pos + 4
                            ));
                        }
                    }
                }

                token::Token::Let => {
                    // Expected Next:
                    // Variable Equals EXPRESSION
                    match (
                        token_iter.next(),
                        token_iter.next(),
                        parse_and_eval_expression(&mut token_iter, &context),
                    ) {
                        (
                            Some(&lexer::TokenAndPos(_, token::Token::Variable(ref variable))),
                            Some(&lexer::TokenAndPos(_, token::Token::Equals)),
                            Ok(ref value),
                        ) => {
                            context
                                .variables
                                .insert(variable.clone().to_string(), value.clone());
                        }
                        (_, _, Err(e)) => {
                            return Err(format!(
                                "At {:?}, {} error in LET expression: {}",
                                line.line_number, pos, e
                            ))
                        }
                        _ => {
                            return Err(format!(
                                "At {:?}, {} invalid syntax for LET.",
                                line.line_number, pos
                            ));
                        }
                    }
                }

                token::Token::Print => {
                    // Expected Next:
                    // EXPRESSION
                    match parse_and_eval_expression(&mut token_iter, &context) {
                        Ok(value::RBasicValue::String(value)) => println!("{}", value),
                        Ok(value::RBasicValue::Number(value)) => println!("{}", value),
                        Ok(value::RBasicValue::Bool(value)) => println!("{}", value),
                        Err(_) => {
                            return Err(format!(
                                "At {:?}. {} PRINT must be followed by valid \
                                                expression",
                                line.line_number, pos
                            ))
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
                            let value = value::RBasicValue::String(input);

                            // Store the string now, can coerce to number later if needed
                            // Can overwrite an existing value
                            context.variables.insert(variable.clone().to_string(), value);
                        }

                        _ => {
                            return Err(format!(
                                "At {:?}, {} INPUT must be followed by a \
                                                variable name",
                                line.line_number,
                                // Adding 5 to put position past INPUT
                                pos + 5
                            ));
                        }
                    }
                }

                token::Token::If => {
                    // Expected Next:
                    // EXPRESSION Then Number
                    // Where Number is a Line Number
                    match (
                        parse_and_eval_expression(&mut token_iter, &context),
                        token_iter.next(),
                        token_iter.next(),
                    ) {
                        (
                            Ok(value::RBasicValue::Bool(ref value)),
                            Some(&lexer::TokenAndPos(_, token::Token::Then)),
                            Some(&lexer::TokenAndPos(_, token::Token::Number(ref number))),
                        ) => {
                            if *value {
                                line_has_goto = true;
                                let n = *number as u32;
                                match line_map.get(&n) {
                                    Some(_target_line) => current_line_number = n,
                                    _ => {
                                        return Err(format!(
                                            "At {:?}, {} invalid target line for \
                                                            IF",
                                            line.line_number, pos
                                        ))
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(format!(
                                "At {:?}, {}, invalid syntax for IF.",
                                line.line_number, pos
                            ));
                        }
                    }
                }

                _ => {
                    return Err(format!("At {:?}, {} invalid syntax", line.line_number, pos));
                }
            }
        }

        // At end of execution, show context:
        // println!("Current context: {:?}", context);

        if !line_has_goto {
            // Move to the next line number in the BTreeMap
            current_line_number = match line_map.range((current_line_number + 1)..).next() {
                Some((&next_line_number, _)) => next_line_number,
                None => break,
            };
        }
    }

    Ok("Completed Successfully".to_string())
}

fn parse_expression(
    token_iter: &mut Peekable<Iter<'_, lexer::TokenAndPos>>,
) -> Result<VecDeque<token::Token>, String> {
    let mut output_queue: VecDeque<token::Token> = VecDeque::new();
    let mut operator_stack: Vec<token::Token> = Vec::new();

    loop {
        match token_iter.peek() {
            Some(&&lexer::TokenAndPos(_, token::Token::Then)) | None => break,
            _ => {}
        }

        match token_iter.next() {
            Some(&lexer::TokenAndPos(_, ref value_token)) if value_token.is_value() && !matches!(value_token, token::Token::BuiltInFn(_)) => {
                output_queue.push_back(value_token.clone())
            }
            Some(&lexer::TokenAndPos(_, token::Token::BuiltInFn(ref func))) => {
                // Push built-in function to operator stack for function call handling
                operator_stack.push(token::Token::BuiltInFn(func.clone()));
            }
            Some(&lexer::TokenAndPos(_, ref op_token)) if op_token.is_operator() => {
                if !operator_stack.is_empty() {
                    let top_op = operator_stack.last().unwrap().clone();
                    if top_op.is_operator() {
                        let associativity = op_token.operator_associavity().unwrap();

                        if (associativity == token::Associativity::Left
                            && op_token.operator_precedence() <= top_op.operator_precedence())
                            || (associativity == token::Associativity::Right
                                && op_token.operator_precedence() < top_op.operator_precedence())
                        {
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
            Some(&lexer::TokenAndPos(_, token::Token::RParen)) => loop {
                match operator_stack.pop() {
                    Some(token::Token::LParen) => break,
                    Some(ref next_token) => output_queue.push_back(next_token.clone()),
                    None => return Err("Mismatched parenthesis in expression".to_string()),
                }
            },
            Some(&lexer::TokenAndPos(_, ref tok)) => {
                return Err(format!("Unexpected token {:?} in expression", tok))
            }
            None => break,
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

fn parse_and_eval_expression<'a>(
    token_iter: &mut Peekable<Iter<'a, lexer::TokenAndPos>>,
    context: &RBasicContext,
) -> Result<value::RBasicValue, String> {
    match parse_expression(token_iter) {
        Ok(mut output_queue) => {
            let mut stack: Vec<value::RBasicValue> = Vec::new();

            // println!("Evaluating queue: {:?}", output_queue);

            while !output_queue.is_empty() {
                match output_queue.pop_front() {
                    Some(token::Token::Number(ref number)) => {
                        stack.push(value::RBasicValue::Number(*number))
                    }
                    Some(token::Token::BString(ref bstring)) => {
                        stack.push(value::RBasicValue::String(bstring.clone()))
                    }
                    Some(token::Token::Variable(ref name)) => match context.variables.get(name) {
                        Some(value) => stack.push(value.clone()),
                        None => {
                            return Err(format!(
                                "Invalid variable reference {} in expression",
                                name
                            ))
                        }
                    },
                    Some(ref unary_token) if unary_token.is_unary_operator() => {
                        if !stack.is_empty() {
                            let value = stack.pop().unwrap();
                            let result = match *unary_token {
                                token::Token::UMinus => -value,
                                token::Token::Bang => !value,
                                // Pattern guard prevents any other match
                                _ => unreachable!(),
                            };
                            match result {
                                Ok(value) => stack.push(value),
                                Err(e) => return Err(e),
                            }
                        } else {
                            return Err(format!("Operator {:?} requires an operand!", unary_token));
                        }
                    }
                    Some(ref comparison_token) if comparison_token.is_comparison_operator() => {
                        if stack.len() >= 2 {
                            let operand2 = &stack.pop().unwrap();
                            let operand1 = &stack.pop().unwrap();

                            let result = match *comparison_token {
                                token::Token::Equals => operand1.eq(operand2),
                                token::Token::NotEqual => operand1.neq(operand2),
                                token::Token::LessThan => operand1.lt(operand2),
                                token::Token::GreaterThan => operand1.gt(operand2),
                                token::Token::LessThanEqual => operand1.lteq(operand2),
                                token::Token::GreaterThanEqual => operand1.gteq(operand2),
                                // Pattern guard prevents any other match
                                _ => unreachable!(),
                            };
                            match result {
                                Ok(value) => stack.push(value::RBasicValue::Bool(value)),
                                Err(e) => return Err(e),
                            }
                        } else {
                            return Err(format!(
                                "Comparison operator {:?} requires two operands",
                                comparison_token
                            ));
                        }
                    }
                    Some(ref binary_op_token) if binary_op_token.is_binary_operator() => {
                        if stack.len() >= 2 {
                            let operand2 = stack.pop().unwrap();
                            let operand1 = stack.pop().unwrap();

                            let result = match *binary_op_token {
                                token::Token::Plus => operand1 + operand2,
                                token::Token::Minus => operand1 - operand2,
                                token::Token::Multiply => operand1 * operand2,
                                token::Token::Divide => operand1 / operand2,
                                token::Token::Modulus => operand1 % operand2,
                                // Pattern guard prevents any other match
                                _ => unreachable!(),
                            };
                            match result {
                                Ok(value) => stack.push(value),
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    Some(token::Token::BuiltInFn(ref func)) => {
                        if stack.len() < 1 {
                            return Err(format!("Function {:?} requires an argument", func));
                        }
                        let arg = stack.pop().unwrap();

                        let result = match func {
                            // Numeric functions
                            token::BuiltInFunction::Sin => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n.sin())),
                                _ => Err(format!("SIN requires a numeric argument")),
                            },
                            token::BuiltInFunction::Cos => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n.cos())),
                                _ => Err(format!("COS requires a numeric argument")),
                            },
                            token::BuiltInFunction::Tan => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n.tan())),
                                _ => Err(format!("TAN requires a numeric argument")),
                            },
                            token::BuiltInFunction::Asin => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n.asin())),
                                _ => Err(format!("ASIN requires a numeric argument")),
                            },
                            token::BuiltInFunction::Acos => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n.acos())),
                                _ => Err(format!("ACOS requires a numeric argument")),
                            },
                            token::BuiltInFunction::Atan => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n.atan())),
                                _ => Err(format!("ATAN requires a numeric argument")),
                            },
                            token::BuiltInFunction::Sqrt => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n.sqrt())),
                                _ => Err(format!("SQRT requires a numeric argument")),
                            },
                            token::BuiltInFunction::Abs => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n.abs())),
                                _ => Err(format!("ABS requires a numeric argument")),
                            },
                            token::BuiltInFunction::Log => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n.ln())),
                                _ => Err(format!("LOG requires a numeric argument")),
                            },
                            token::BuiltInFunction::Exp => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n.exp())),
                                _ => Err(format!("EXP requires a numeric argument")),
                            },
                            token::BuiltInFunction::Floor => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n.floor())),
                                _ => Err(format!("FLOOR requires a numeric argument")),
                            },
                            token::BuiltInFunction::Ceil => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n.ceil())),
                                _ => Err(format!("CEIL requires a numeric argument")),
                            },
                            token::BuiltInFunction::Round => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n.round())),
                                _ => Err(format!("ROUND requires a numeric argument")),
                            },
                            token::BuiltInFunction::Rand => match arg {
                                value::RBasicValue::Number(n) => {
                                    if n.fract() != 0.0 {
                                        return Err("RAND requires an integer argument".to_string());
                                    }

                                    if n < 0.0 {
                                        return Err("RAND requires a non-negative integer".to_string());
                                    }

                                    Ok(value::RBasicValue::Number(
                                        rand::rng().random_range(0..=n as i64) as f64
                                    ))
                                }
                                _ => Err(format!("RAND requires a numeric argument")),
                            },
                            token::BuiltInFunction::Num => match arg {
                                value::RBasicValue::Number(n) => Ok(value::RBasicValue::Number(n)),
                                value::RBasicValue::String(s) => {
                                    match f64::from_str(&s) {
                                        Ok(n) => Ok(value::RBasicValue::Number(n)),
                                        Err(_) => Err(format!("NUM: cannot parse '{}' as a number", s)),
                                    }
                                }
                                value::RBasicValue::Bool(b) => {
                                    Ok(value::RBasicValue::Number(if b { 1.0 } else { 0.0 }))
                                }
                            },
                            token::BuiltInFunction::Str => match arg {
                                value::RBasicValue::Number(n) => {
                                    // Remove trailing zeros for cleaner output
                                    if n.fract() == 0.0 && n.abs() < 1e15 {
                                        Ok(value::RBasicValue::String((n as i64).to_string()))
                                    } else {
                                        Ok(value::RBasicValue::String(n.to_string()))
                                    }
                                }
                                value::RBasicValue::String(s) => {
                                    Ok(value::RBasicValue::String(s))
                                }
                                value::RBasicValue::Bool(b) => {
                                    Ok(value::RBasicValue::String(
                                        if b { "true".to_string() } else { "false".to_string() }
                                    ))
                                }
                            },
                            // String functions
                            token::BuiltInFunction::Len => match arg {
                                value::RBasicValue::String(s) => Ok(value::RBasicValue::Number(s.len() as f64)),
                                _ => Err(format!("LEN requires a string argument")),
                            },
                            token::BuiltInFunction::Chr => match arg {
                                value::RBasicValue::Number(n) => {
                                    let ch = std::char::from_u32(n as u32)
                                        .ok_or_else(|| format!("CHR: invalid character code {}", n))?;
                                    Ok(value::RBasicValue::String(ch.to_string()))
                                },
                                _ => Err(format!("CHR requires a numeric argument")),
                            },
                            token::BuiltInFunction::Asc => match arg {
                                value::RBasicValue::String(s) => {
                                    let code = s.chars().next()
                                        .map(|c| c as u32 as f64)
                                        .ok_or_else(|| "ASC: empty string".to_string())?;
                                    Ok(value::RBasicValue::Number(code))
                                },
                                _ => Err(format!("ASC requires a string argument")),
                            },
                        };

                        match result {
                            Ok(value) => stack.push(value),
                            Err(e) => return Err(e),
                        }
                    }
                    None => unreachable!(),
                    _ => unreachable!(),
                }
            }

            if stack.len() == 1 {
                Ok(stack.pop().unwrap())
            } else {
                Err("Invalid expression: unbalanced operands and operators".to_string())
            }
        }

        _ => Err("Invalid expression!".to_string()),
    }
}
