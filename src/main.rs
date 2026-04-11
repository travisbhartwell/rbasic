use std::io::{self, Read, Write};
use std::fs::File;
use std::env;

use std::collections::BTreeMap;

use rbasic::lexer;
use rbasic::evaluator;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;  // `s` contains the contents of "foo.txt"
    Ok(s)
}

fn main() {
    let mut argv = env::args();

    if env::args().len() > 1 {
        let program: String = argv.nth(1).unwrap();
        match read_file(program.as_str()) {
            Ok(s) => {
                let mut code_lines: Vec<lexer::LineOfCode> = Vec::new();

                for (lineno, line) in s.lines().enumerate() {
                    let result = lexer::tokenize_line(line, false);
                    match result {
                        Ok(x) => {
                            // println!("{}", line);
                            // println!("Line Number: {:?}", x.line_number);
                            // println!("Tokens: {:?}", x.tokens);
                            code_lines.push(x)
                        }
                        Err(e) => println!("Error at line {}: {}", lineno, e),
                    }
                }

                match evaluator::evaluate(code_lines) {
                    Ok(msg) => println!("{}", msg),
                    Err(msg) => println!("Execution failed: {}", msg),
                }

            }
            Err(err) => println!("Getting file contents failed with error: {}", err),
        };
    } else {
        let mut code_lines: Vec<lexer::LineOfCode> = Vec::new();
        let mut stdout = io::stdout();

        println!("RBASIC Interactive Shell");
        println!("Type 'RUN' to execute program, 'LIST' to view, 'CLEAR' to reset, 'QUIT' to exit");

        loop {
            print!("] ");
            stdout.flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    println!();
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break;
                }
            }

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            match input.to_uppercase().as_str() {
                "QUIT" | "EXIT" => break,
                "RUN" => {
                    match evaluator::evaluate(code_lines.clone()) {
                        Ok(msg) => println!("{}", msg),
                        Err(msg) => println!("Execution failed: {}", msg),
                    }
                    continue;
                }
                "LIST" => {
                    let mut lines = BTreeMap::new();
                    evaluator::get_line_map(&code_lines, &mut lines);
                    for (_, line) in &lines {
                        if let Some(l) = &line.text {
                            println!("{}", l);
                        }
                    }
                    continue;
                }
                "CLEAR" => {
                    code_lines.clear();
                    println!("Program cleared.");
                    continue;
                }
                _ => {}
            }

            match lexer::tokenize_line(input, true) {
                Ok(line_of_code) => {
                    code_lines.push(line_of_code);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}
