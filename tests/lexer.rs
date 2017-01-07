extern crate rbasic;

use rbasic::lexer::*;
use rbasic::token::*;

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
    let tokens: Vec<TokenAndPos> = vec![TokenAndPos(3, Token::Print),
                                        TokenAndPos(9, Token::BString("FOO BAR BAZ".to_string()))];
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
