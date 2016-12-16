use lexer;
use std::collections::BTreeMap;

pub fn evaluate(code_lines: Vec<lexer::LineOfCode>) -> Result<String, String> {
    let mut line_map = BTreeMap::new();

    for line in code_lines {
        line_map.insert(line.line_number, line.tokens);
    }

    let line_numbers: Vec<_> = line_map.keys().cloned().collect();
    let num_lines = line_numbers.len();
    let mut line_index = 0;
    // TODO: Feels hacky
    let mut line_has_goto = false;

    loop {
        // match line_numbers.iter().position(|item| *item == lexer::LineNumber(5)) {
        //     Some(pos) => {
        //         println!("Found at pos {}, element: {:?}", pos, line_numbers[pos]);
        //     }
        //     None => println!("Not found!"),
        // }

        let line_number = &line_numbers[line_index];
        let tokens = line_map.get(&line_number).unwrap();
        let mut token_iter = tokens.iter().peekable();

        while token_iter.peek() != None {
            let lexer::TokenAndPos(pos, ref token) = *token_iter.next().unwrap();
            // Set default value
            line_has_goto = false;

            match *token {
                lexer::Token::Goto => {
                    line_has_goto = true;
                    match token_iter.next() {
                        Some(&lexer::TokenAndPos(_, lexer::Token::Number(number))) => {
                            let n = lexer::LineNumber(number as u32);
                            if line_map.get(&n).is_some() {
                                line_index = line_numbers.iter()
                                    .position(|item| *item == n)
                                    .unwrap();
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
                _ => println!("At {}, {:?}", pos, token),
            }
        }

        if !line_has_goto {
            line_index += 1;
            if line_index == num_lines {
                break;
            }
        }
    }

    Ok("Completed Successfully".to_string())
}
