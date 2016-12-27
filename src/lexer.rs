use token;

extern crate itertools;
use itertools::Itertools;

use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineNumber(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct TokenAndPos(pub u32, pub token::Token);

#[derive(Debug, Clone, PartialEq)]
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
            match ch {
                ch if ch.is_whitespace() => {
                    // Skip whitespace
                    continue;
                }

                // At the beginning of a string
                '"' => {
                    // TODO: Handle escaped quotes
                    // TODO: Handle malformed string
                    let str_chars: Vec<char> = char_iter.by_ref()
                        .take_while(|&(_, x)| x != '"')
                        .map(|(_, x)| x)
                        .collect();
                    let bstring: String = str_chars.into_iter().collect();
                    tokens.push(TokenAndPos(pos, token::Token::BString(bstring)))
                }
                '-' => {
                    if !tokens.is_empty() && tokens.last().unwrap().1.is_value() {
                        tokens.push(TokenAndPos(pos, token::Token::Minus))
                    } else {
                        tokens.push(TokenAndPos(pos, token::Token::UMinus))
                    }
                }
                '!' => tokens.push(TokenAndPos(pos, token::Token::Bang)),
                '(' => tokens.push(TokenAndPos(pos, token::Token::LParen)),
                ')' => tokens.push(TokenAndPos(pos, token::Token::RParen)),
                _ => {
                    // Otherwise, next token is until next whitespace or closing paren
                    let mut token_chars: Vec<char> = char_iter.by_ref()
                        .peeking_take_while(|&(_, x)| !(x.is_whitespace() || x == ')'))
                        .map(|(_, x)| x)
                        .collect();
                    token_chars.insert(0, ch);
                    let token_str: String = token_chars.into_iter().collect();

                    if i32::from_str(token_str.as_str()).is_ok() {
                        tokens.push(TokenAndPos(pos,
                                          token::Token::Number(i32::from_str(token_str.as_str())
                                              .unwrap())));
                    } else {
                        let token = token::Token::token_for_string(token_str.as_str());

                        match token {
                            None => {
                                if is_valid_identifier(&token_str) {
                                    tokens
                                    .push(TokenAndPos(pos,
                                                      token::Token::Variable(token_str
                                                                             .to_string())))
                                } else {
                                    return Err(format!("Unimplemented token at {}:\t{}",
                                                       pos,
                                                       token_str));
                                }
                            }

                            Some(token::Token::Rem) => {
                                tokens.push(TokenAndPos(pos, token::Token::Rem));
                                // Skip the space after REM
                                char_iter.next();
                                // The rest of the line is a comment
                                let comment_str: String =
                                    char_iter.by_ref().map(|(_, x)| x).collect();
                                tokens.push(TokenAndPos((pos + 4) as u32,
                                                        token::Token::Comment(comment_str)))
                            }

                            Some(token) => {
                                tokens.push(TokenAndPos(pos, token));
                            }
                        }
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
        let tokens: Vec<TokenAndPos> = vec![TokenAndPos(3, token::Token::Goto),
                                            TokenAndPos(8, token::Token::Number(100))];
        assert_eq!(tokens, line_of_code.tokens)
    }

    #[test]
    fn tokenize_line_with_string() {
        let line_of_code = tokenize_line("10 PRINT \"FOO BAR BAZ\"").unwrap();
        assert_eq!(LineNumber(10), line_of_code.line_number);
        let tokens: Vec<TokenAndPos> =
            vec![TokenAndPos(3, token::Token::Print),
                 TokenAndPos(9, token::Token::BString("FOO BAR BAZ".to_string()))];
        assert_eq!(tokens, line_of_code.tokens)
    }

    #[test]
    fn tokenize_line_with_identifier() {
        let line_of_code = tokenize_line("10 INPUT A").unwrap();
        assert_eq!(LineNumber(10), line_of_code.line_number);
        let tokens: Vec<TokenAndPos> = vec![TokenAndPos(3, token::Token::Input),
                                            TokenAndPos(9,
                                                        token::Token::Variable("A".to_string()))];
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
            vec![TokenAndPos(3, token::Token::Rem),
                 TokenAndPos(7,
                             token::Token::Comment("THIS IS A COMMENT 123".to_string()))];
        assert_eq!(tokens, line_of_code.tokens)
    }
}
