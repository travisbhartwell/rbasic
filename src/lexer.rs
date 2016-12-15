// 5  REM inputting the argument
// 10  PRINT " factorial of:"
// 20  INPUT A
// 30  LET B = 1
// 35  REM beginning of the loop
// 40  IF A <= 1 THEN 80
// 50  LET B = B * A
// 60  LET A = A - 1
// 70  GOTO 40
// 75  REM prints the result
// 80  PRINT B
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct LineNumber(u32);

#[derive(Debug, PartialEq)]
pub enum Token {
    Comment(String),

    // Variables and Literals
    Variable(String),
    Number(i32),
    BString(String),

    // Operators
    Equals,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    NotEqual,
    Multiply,
    Divide,
    Minus,
    Plus,

    // Keywords
    Goto,
    If,
    Input,
    Let,
    Print,
    Rem,
    Then,
}

#[derive(Debug)]
pub struct LineOfCode {
    line_number: LineNumber,
    tokens: Vec<Token>,
}

pub fn tokenize_line(line: &str) -> Result<LineOfCode, String> {
    let words = line.split_whitespace();
    let mut line_number = LineNumber(0);
    let mut tokens: Vec<Token> = Vec::new();

    for (word_number, word) in words.enumerate() {
        if word.chars().all(char::is_numeric) {
            if word_number == 0 {
                line_number = LineNumber(u32::from_str(word).unwrap())
            } else {
                tokens.push(Token::Number(i32::from_str(word).unwrap()))
            }
        } else {
            if word_number == 0 {
                return Err(format!("Line must start with a line number:\n\t{}", line));
            }

            match word {
                // Match keywords
                "GOTO" => tokens.push(Token::Goto),
                "IF" => tokens.push(Token::If),
                "INPUT" => tokens.push(Token::Input),
                "LET" => tokens.push(Token::Let),
                "PRINT" => tokens.push(Token::Print),
                "REM" => tokens.push(Token::Rem),
                "THEN" => tokens.push(Token::Then),

                // Operators
                "=" => tokens.push(Token::Equals),
                "<" => tokens.push(Token::LessThan),
                ">" => tokens.push(Token::GreaterThan),
                "<=" => tokens.push(Token::LessThanEqual),
                ">=" => tokens.push(Token::GreaterThanEqual),
                "<>" | "><" => tokens.push(Token::NotEqual),
                "*" => tokens.push(Token::Multiply),
                "/" => tokens.push(Token::Divide),
                "-" => tokens.push(Token::Minus),
                "+" => tokens.push(Token::Plus),
                _ => return Err(format!("Unimplemented token:\t{}", word)),
            }
        }
    }

    Ok(LineOfCode {
        line_number: line_number,
        tokens: tokens,
    })
}

#[cfg(test)]
mod tests {
    use lexer;

    #[test]
    fn tokenize_line() {
        let line_of_code = lexer::tokenize_line("10 GOTO 100").unwrap();
        assert_eq!(lexer::LineNumber(10), line_of_code.line_number);
        let tokens: Vec<lexer::Token> = vec![lexer::Token::Goto, lexer::Token::Number(100)];
        assert_eq!(tokens, line_of_code.tokens)
    }

    #[test]
    #[ignore]
    fn tokenize_comment_line() {
        let line_of_code = lexer::tokenize_line("5  REM THIS IS A COMMENT 123").unwrap();
        assert_eq!(lexer::LineNumber(5), line_of_code.line_number);
        let tokens: Vec<lexer::Token> =
            vec![lexer::Token::Rem, lexer::Token::Comment("THIS IS A COMMENT 123".to_string())];
        assert_eq!(tokens, line_of_code.tokens)
    }

    #[test]
    fn tokenize_invalid_line() {
        let line_of_code = lexer::tokenize_line("REM Invalid Line");
        assert!(line_of_code.is_err());
    }
}
