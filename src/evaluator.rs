use lexer;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum VariableValue {
    String(String),
    Number(i32),
}

#[derive(Debug)]
struct RBasicContext {
    variables: HashMap<String, VariableValue>,
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

        println!("Looking at line: {:?}", line_number);
        if !tokens.is_empty() {
            let lexer::TokenAndPos(pos, ref token) = *token_iter.next().unwrap();
            // Set default value
            line_has_goto = false;

            match *token {
                lexer::Token::Rem => {
                    // Skip the rest of the line so do nothing
                }

                lexer::Token::Goto => {
                    line_has_goto = true;
                    match token_iter.next() {
                        Some(&lexer::TokenAndPos(pos, lexer::Token::Number(number))) => {
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

                lexer::Token::Let => {
                    // Expected Next:
                    // Variable Equals EXPRESSION
                }

                lexer::Token::Print => {
                    // Expected Next:
                    // EXPRESSION
                }

                lexer::Token::Input => {
                    match token_iter.next() {
                        Some(&lexer::TokenAndPos(_, lexer::Token::Variable(ref variable))) => {
                            let mut input = String::new();

                            io::stdin()
                                .read_line(&mut input)
                                .expect("failed to read line");

                            // Store the string now, can coerce to number later if needed
                            // Can overwrite an existing value
                            context.variables
                                .entry(variable.clone().to_string())
                                .or_insert(VariableValue::String(input.trim().to_string()));
                        }

                        _ => {
                            return Err(format!("At {:?}, {} INPUT must be followed by a \
                                                variable name",
                                               line_number,
                                               pos));
                        }
                    }
                }

                lexer::Token::If => {
                    // Expected Next:
                    // EXPRESSION Then Number
                }

                _ => {
                    return Err(format!("At {:?}, {} invalid syntax", line_number, pos));
                }
            }
        }

        // At end of execution, show context:
        println!("Current context: {:?}", context);

        if !line_has_goto {
            line_index += 1;
            if line_index == num_lines {
                break;
            }
        }
    }

    Ok("Completed Successfully".to_string())
}
