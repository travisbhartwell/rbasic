use rbasic::lexer::*;
use rbasic::token::*;

// ============================================
// Line Number Parsing
// ============================================

#[test]
fn tokenize_no_line_number() {
    assert!(tokenize_line("REM Invalid Line", false).is_err());
}

#[test]
fn tokenize_bad_line_number() {
    assert!(tokenize_line("10B REM Invalid Line", false).is_err());
}

#[test]
fn tokenize_line_numbers() {
    assert_eq!(tokenize_line("0 PRINT \"Zero\"", false).unwrap().line_number, 0);
    assert_eq!(tokenize_line("99999 PRINT \"Large\"", false).unwrap().line_number, 99999);
}

#[test]
fn tokenize_leading_whitespace_fails() {
    assert!(tokenize_line("  10 PRINT \"Test\"", false).is_err());
}

// ============================================
// GOTO Token
// ============================================

#[test]
fn tokenize_goto() {
    let loc = tokenize_line("10 GOTO 100", false).unwrap();
    assert_eq!(loc.line_number, 10);
    assert_eq!(loc.tokens, vec![
        TokenAndPos(3, Token::Goto),
        TokenAndPos(8, Token::Number(100.0))
    ]);
}

// ============================================
// String Tokenization
// ============================================

#[test]
fn tokenize_strings() {
    let loc = tokenize_line("10 PRINT \"FOO BAR BAZ\"", false).unwrap();
    assert_eq!(loc.tokens, vec![
        TokenAndPos(3, Token::Print),
        TokenAndPos(9, Token::BString("FOO BAR BAZ".to_string()))
    ]);
}

#[test]
fn tokenize_empty_string() {
    let loc = tokenize_line("10 PRINT \"\"", false).unwrap();
    assert!(loc.tokens.iter().any(|t| matches!(t, TokenAndPos(_, Token::BString(s)) if s.is_empty())));
}

// ============================================
// Variables/Identifiers
// ============================================

#[test]
fn tokenize_identifier() {
    let loc = tokenize_line("10 INPUT A", false).unwrap();
    assert_eq!(loc.tokens, vec![
        TokenAndPos(3, Token::Input),
        TokenAndPos(9, Token::Variable("A".to_string()))
    ]);
}

#[test]
fn tokenize_bad_identifier() {
    assert!(tokenize_line("10 INPUT `A", false).is_err());
}

#[test]
fn tokenize_identifier_variants() {
    let loc = tokenize_line("10 LET MY_VAR = 5", false).unwrap();
    assert!(loc.tokens.iter().any(|t| matches!(t, TokenAndPos(_, Token::Variable(ref v)) if v == "MY_VAR")));
    assert!(loc.tokens.iter().any(|t| matches!(t, TokenAndPos(_, Token::Number(5.0)))));
}

// ============================================
// Comments
// ============================================

#[test]
fn tokenize_comment() {
    let loc = tokenize_line("5  REM THIS IS A COMMENT", false).unwrap();
    assert_eq!(loc.tokens.len(), 2);
    assert!(matches!(loc.tokens[0], TokenAndPos(_, Token::Rem)));
    assert!(matches!(loc.tokens[1], TokenAndPos(_, Token::Comment(ref s)) if s.contains("COMMENT")));
}

#[test]
fn tokenize_rem_only() {
    let loc = tokenize_line("10 REM", false).unwrap();
    assert!(loc.tokens.len() >= 1);
    assert!(matches!(loc.tokens[0], TokenAndPos(_, Token::Rem)));
}

// ============================================
// Numbers
// ============================================

#[test]
fn tokenize_numbers() {
    assert!(tokenize_line("10 LET X = 42", false).unwrap()
        .tokens.iter().any(|t| matches!(t, TokenAndPos(_, Token::Number(42.0)))));
    assert!(tokenize_line("10 LET X = 3.14", false).unwrap()
        .tokens.iter().any(|t| matches!(t, TokenAndPos(_, Token::Number(3.14)))));
}

#[test]
fn tokenize_minus_unary_vs_binary() {
    // UMinus at start of expression
    let loc1 = tokenize_line("10 LET X = -5", false).unwrap();
    assert!(loc1.tokens.iter().any(|t| matches!(t, TokenAndPos(_, Token::UMinus))));
    
    // Minus (binary) after value
    let loc2 = tokenize_line("10 LET X = 10 - 5", false).unwrap();
    assert!(loc2.tokens.iter().any(|t| matches!(t, TokenAndPos(_, Token::Minus))));
}

// ============================================
// Operators
// ============================================

#[test]
fn tokenize_arithmetic_operators() {
    let loc = tokenize_line("10 LET X = 1 + 2 * 3 / 4 % 5", false).unwrap();
    let tokens: Vec<&Token> = loc.tokens.iter().map(|t| &t.1).collect();
    assert!(tokens.contains(&&Token::Plus));
    assert!(tokens.contains(&&Token::Multiply));
    assert!(tokens.contains(&&Token::Divide));
    assert!(tokens.contains(&&Token::Modulus));
}

#[test]
fn tokenize_comparison_operators() {
    let loc = tokenize_line("10 IF X <= 5 THEN 100", false).unwrap();
    assert!(loc.tokens.iter().any(|t| matches!(t, TokenAndPos(_, Token::LessThanEqual))));
    assert!(loc.tokens.iter().any(|t| matches!(t, TokenAndPos(_, Token::Then))));
}

#[test]
fn tokenize_not_equal() {
    assert!(tokenize_line("10 IF X <> 5 THEN 100", false).unwrap()
        .tokens.iter().any(|t| matches!(t, TokenAndPos(_, Token::NotEqual))));
}

#[test]
fn tokenize_parens_and_bang() {
    let loc = tokenize_line("10 LET X = (1 + 2) * !Y", false).unwrap();
    assert!(loc.tokens.iter().any(|t| matches!(t, TokenAndPos(_, Token::LParen))));
    assert!(loc.tokens.iter().any(|t| matches!(t, TokenAndPos(_, Token::RParen))));
    assert!(loc.tokens.iter().any(|t| matches!(t, TokenAndPos(_, Token::Bang))));
}

// ============================================
// Keywords
// ============================================

#[test]
fn tokenize_keywords() {
    let keywords = vec![
        ("GOTO", Token::Goto),
        ("IF", Token::If),
        ("INPUT", Token::Input),
        ("LET", Token::Let),
        ("PRINT", Token::Print),
        ("REM", Token::Rem),
        ("THEN", Token::Then),
    ];
    
    for (keyword, expected_token) in keywords {
        let loc = tokenize_line(&format!("10 {}", keyword), false).unwrap();
        assert!(
            loc.tokens.iter().any(|t| t.1 == expected_token),
            "Failed to tokenize keyword: {}", keyword
        );
    }
}

// ============================================
// Built-in Functions
// ============================================

#[test]
fn tokenize_builtins() {
    let builtins = vec![
        "SIN", "COS", "TAN", "SQRT", "ABS", "LOG", "EXP",
        "FLOOR", "CEIL", "ROUND", "RAND", "NUM", "STR",
        "LEN", "CHR", "ASC"
    ];
    
    for builtin in builtins {
        let loc = tokenize_line(&format!("10 PRINT {}", builtin), false).unwrap();
        assert!(
            loc.tokens.iter().any(|t| matches!(&t.1, Token::BuiltInFn(_))),
            "Failed to tokenize builtin: {}", builtin
        );
    }
}

// ============================================
// Store Text Option
// ============================================

#[test]
fn tokenize_store_text() {
    let loc_with = tokenize_line("10 PRINT \"Hello\"", true).unwrap();
    assert!(loc_with.text.is_some());
    
    let loc_without = tokenize_line("10 PRINT \"Hello\"", false).unwrap();
    assert!(loc_without.text.is_none());
}

// ============================================
// Edge Cases
// ============================================

#[test]
fn tokenize_empty_after_number() {
    let loc = tokenize_line("10", false).unwrap();
    assert_eq!(loc.line_number, 10);
    assert!(loc.tokens.is_empty());
}

#[test]
fn tokenize_multiple_spaces() {
    assert_eq!(tokenize_line("10   LET    X   =   5", false).unwrap().tokens.len(), 4);
}

#[test]
fn tokenize_complex_expression() {
    let loc = tokenize_line("10 LET X = (A + B) * (C - D) / E", false).unwrap();
    assert_eq!(loc.line_number, 10);
    assert!(loc.tokens.len() > 10);
}
