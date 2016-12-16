use std::io::Read;
use std::fs::File;

mod lexer;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;  // `s` contains the contents of "foo.txt"
    Ok(s)
}

fn main() {
    match read_file("test.bas") {
        Ok(s) => {
            for (lineno, line) in s.lines().enumerate() {
                let result = lexer::tokenize_line(line);
                match result {
                    Ok(x) => {
                        println!("{}", line);
                        println!("Line Number: {:?}", x.line_number);
                        println!("Tokens: {:?}", x.tokens)
                    }
                    Err(e) => println!("Error at line {}: {}", lineno, e),
                }
            }
        }
        Err(err) => println!("Getting file contents failed with error: {}", err),
    };
}
