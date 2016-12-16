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

#[derive(Debug, PartialEq)]
pub struct TokenAndPos(u32, Token);

#[derive(Debug, PartialEq)]
pub struct LineOfCode {
    pub line_number: LineNumber,
    pub tokens: Vec<TokenAndPos>,
}

pub fn tokenize_line(line: &str) -> Result<LineOfCode, String> {
    let mut char_iter = line.chars().enumerate().peekable();
    let mut line_number = LineNumber(0);
    let mut tokens: Vec<TokenAndPos> = Vec::new();

    while char_iter.peek() != None {
        let (pos, ch) = char_iter.next().unwrap();
        let pos = pos as u32;

        if pos == 0 {
            if ch.is_numeric() {
                let mut num_chars: Vec<char> = char_iter.by_ref()
                    .take_while(|&(_, x)| !x.is_whitespace())
                    .map(|(_, x)| x)
                    .collect();
                num_chars.insert(0, ch);
                let num_str: String = num_chars.into_iter().collect();

                match u32::from_str(num_str.as_str()) {
                    Ok(number) => line_number = LineNumber(number),
                    Err(_) => {
                        return Err(format!("Line must start with number followed by \
                                            whitespace:\n\t{}",
                                           line))
                    }
                };
            } else {
                return Err(format!("Line must start with a line number:\n\t{}", line));
            }
        } else {
            if ch.is_whitespace() {
                // Skip whitespace
                continue;
            }

            // At the beginning of a string
            if ch == '"' {
                // TODO: Handle escaped quotes
                // TODO: Handle malformed string
                let mut str_chars: Vec<char> = char_iter.by_ref()
                    .take_while(|&(_, x)| x != '"')
                    .map(|(_, x)| x)
                    .collect();
                str_chars.push('"');
                str_chars.insert(0, ch);
                let bstring: String = str_chars.into_iter().collect();
                tokens.push(TokenAndPos(pos, Token::BString(bstring)))
            } else {

                // Otherwise, next token is until next whitespace
                let mut token_chars: Vec<char> = char_iter.by_ref()
                    .take_while(|&(_, x)| !x.is_whitespace())
                    .map(|(_, x)| x)
                    .collect();
                token_chars.insert(0, ch);
                let token_str: String = token_chars.into_iter().collect();

                if token_str.chars().all(char::is_numeric) {
                    tokens.push(TokenAndPos(pos,
                                            Token::Number(i32::from_str(token_str.as_str())
                                                .unwrap())));
                } else {
                    match token_str.as_str() {
                        // Match keywords
                        "GOTO" => tokens.push(TokenAndPos(pos, Token::Goto)),
                        "IF" => tokens.push(TokenAndPos(pos, Token::If)),
                        "INPUT" => tokens.push(TokenAndPos(pos, Token::Input)),
                        "LET" => tokens.push(TokenAndPos(pos, Token::Let)),
                        "PRINT" => tokens.push(TokenAndPos(pos, Token::Print)),
                        "REM" => {
                            tokens.push(TokenAndPos(pos, Token::Rem));
                            // The rest of the line is a comment
                            let comment_str: String = char_iter.by_ref().map(|(_, x)| x).collect();
                            tokens.push(TokenAndPos((pos + 4) as u32, Token::Comment(comment_str)))
                        }
                        "THEN" => tokens.push(TokenAndPos(pos, Token::Then)),

                        // Operators
                        "=" => tokens.push(TokenAndPos(pos, Token::Equals)),
                        "<" => tokens.push(TokenAndPos(pos, Token::LessThan)),
                        ">" => tokens.push(TokenAndPos(pos, Token::GreaterThan)),
                        "<=" => tokens.push(TokenAndPos(pos, Token::LessThanEqual)),
                        ">=" => tokens.push(TokenAndPos(pos, Token::GreaterThanEqual)),
                        "<>" | "><" => tokens.push(TokenAndPos(pos, Token::NotEqual)),
                        "*" => tokens.push(TokenAndPos(pos, Token::Multiply)),
                        "/" => tokens.push(TokenAndPos(pos, Token::Divide)),
                        "-" => tokens.push(TokenAndPos(pos, Token::Minus)),
                        "+" => tokens.push(TokenAndPos(pos, Token::Plus)),
                        token_str if is_valid_identifier(token_str) => {
                            tokens.push(TokenAndPos(pos, Token::Variable(token_str.to_string())))
                        }
                        _ => return Err(format!("Unimplemented token at {}:\t{}", pos, token_str)),
                    }
                }
            }
        }
    }

    Ok(LineOfCode {
        line_number: line_number,
        tokens: tokens,
    })
}

#[allow(unused_variables)]
// Starts with [a-zA-Z_]
// Followed by any number of [a-zA-Z0-9_]
fn is_valid_identifier(token_str: &str) -> bool {
    let mut v = token_str.chars();
    let c = v.next();
    match c {
        Some(c) => {
            match c {
                'a'...'z' | 'A'...'Z' => (),
                _ => return false,
            }
        }
        None => return false,
    }
    for c in v {
        match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => (),
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use lexer::*;
    #[test]
    fn tokenize_no_line_number() {
        let line_of_code = tokenize_line("REM Invalid Line");
        assert!(line_of_code.is_err());
    }

    #[test]
    fn tokenize_bad_line_number() {
        let line_of_code = tokenize_line("10B REM Invalid Line");
        assert!(line_of_code.is_err());
    }

    #[test]
    fn tokenize_line_with_goto() {
        let line_of_code = tokenize_line("10 GOTO 100").unwrap();
        assert_eq!(LineNumber(10), line_of_code.line_number);
        let tokens: Vec<TokenAndPos> = vec![TokenAndPos(3, Token::Goto),
                                            TokenAndPos(8, Token::Number(100))];
        assert_eq!(tokens, line_of_code.tokens)
    }

    #[test]
    fn tokenize_line_with_string() {
        let line_of_code = tokenize_line("10 PRINT \"FOO BAR BAZ\"").unwrap();
        assert_eq!(LineNumber(10), line_of_code.line_number);
        let tokens: Vec<TokenAndPos> =
            vec![TokenAndPos(3, Token::Print),
                 TokenAndPos(9, Token::BString("\"FOO BAR BAZ\"".to_string()))];
        assert_eq!(tokens, line_of_code.tokens)
    }

    #[test]
    fn tokenize_line_with_identifier() {
        let line_of_code = tokenize_line("10 INPUT A").unwrap();
        assert_eq!(LineNumber(10), line_of_code.line_number);
        let tokens: Vec<TokenAndPos> = vec![TokenAndPos(3, Token::Input),
                                            TokenAndPos(9, Token::Variable("A".to_string()))];
        assert_eq!(tokens, line_of_code.tokens)
    }

    #[test]
    fn tokenize_line_with_bad_identifier() {
        let line_of_code = tokenize_line("10 INPUT `A");
        assert!(line_of_code.is_err());
    }

    #[test]
    fn tokenize_line_with_comment() {
        let line_of_code = tokenize_line("5  REM THIS IS A COMMENT 123").unwrap();
        assert_eq!(LineNumber(5), line_of_code.line_number);
        let tokens: Vec<TokenAndPos> =
            vec![TokenAndPos(3, Token::Rem),
                 TokenAndPos(7, Token::Comment("THIS IS A COMMENT 123".to_string()))];
        assert_eq!(tokens, line_of_code.tokens)
    }
}
