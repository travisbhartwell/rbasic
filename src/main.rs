extern crate itertools;
extern crate rbasic;

use std::io::Read;
use std::fs::File;
use std::env;

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
                    let result = lexer::tokenize_line(line);
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
    }
}
