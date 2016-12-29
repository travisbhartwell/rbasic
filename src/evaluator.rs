use lexer;
use token;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::slice::Iter;
use std::io;

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
        let mut token_iter = tokens.iter();

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

fn parse_and_eval_expression<'a>(mut token_iter: &mut Iter<'a, lexer::TokenAndPos>,
                                 context: &RBasicContext)
                                 -> Result<RBasicValue, String> {
    let mut result: RBasicValue = RBasicValue::String(String::new());

    match token_iter.next() {
        Some(&lexer::TokenAndPos(_, token::Token::Number(number))) => {
            result = RBasicValue::Number(number);
        }
        Some(&lexer::TokenAndPos(pos, token::Token::Variable(ref variable))) => {
            match context.variables.get(variable) {
                Some(value) => result = value.clone(),
                None => {
                    return Err(format!("At {}, invalid variable reference in expression: {}",
                                       pos,
                                       variable))
                }
            }
        }
        Some(&lexer::TokenAndPos(_, token::Token::BString(ref string))) => {
            result = RBasicValue::String(string.clone());
        }
        // Unary Minus
        Some(&lexer::TokenAndPos(pos, token::Token::Minus)) => {
            match parse_and_eval_expression(&mut token_iter, &context) {
                Ok(RBasicValue::Number(number)) => {
                    result = RBasicValue::Number(-number);
                }
                Ok(_) => return Err(format!("At {}, can only negate numerical values", pos)),
                Err(e) => return Err(e),
            }
        }
        // Unary Not
        Some(&lexer::TokenAndPos(pos, token::Token::Bang)) => {
            match parse_and_eval_expression(&mut token_iter, &context) {
                Ok(RBasicValue::Bool(value)) => {
                    result = RBasicValue::Bool(!value);
                }
                Ok(_) => {
                    return Err(format!("At {}, boolean not only works against boolean values", pos))
                }
                Err(e) => return Err(e),
            }
        }

        None => {
            // Empty Expression
        }

        _ => println!("Unimplemented!"),
    }


    Ok(result)
}
