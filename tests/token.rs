use rbasic::token::*;

// ============================================
// Token::token_for_string Tests
// ============================================

#[test]
fn test_token_for_operators() {
    // Comparison
    assert_eq!(Token::token_for_string("="), Some(Token::Equals));
    assert_eq!(Token::token_for_string("<"), Some(Token::LessThan));
    assert_eq!(Token::token_for_string("<="), Some(Token::LessThanEqual));
    assert_eq!(Token::token_for_string("<>"), Some(Token::NotEqual));
    
    // Arithmetic
    assert_eq!(Token::token_for_string("+"), Some(Token::Plus));
    assert_eq!(Token::token_for_string("-"), Some(Token::Minus));
    assert_eq!(Token::token_for_string("*"), Some(Token::Multiply));
    assert_eq!(Token::token_for_string("/"), Some(Token::Divide));
    assert_eq!(Token::token_for_string("%"), Some(Token::Modulus));
    
    // Parens and bang
    assert_eq!(Token::token_for_string("("), Some(Token::LParen));
    assert_eq!(Token::token_for_string(")"), Some(Token::RParen));
    assert_eq!(Token::token_for_string("!"), Some(Token::Bang));
}

#[test]
fn test_token_for_keywords() {
    assert_eq!(Token::token_for_string("GOTO"), Some(Token::Goto));
    assert_eq!(Token::token_for_string("IF"), Some(Token::If));
    assert_eq!(Token::token_for_string("LET"), Some(Token::Let));
    assert_eq!(Token::token_for_string("PRINT"), Some(Token::Print));
    assert_eq!(Token::token_for_string("REM"), Some(Token::Rem));
    assert_eq!(Token::token_for_string("THEN"), Some(Token::Then));
    assert_eq!(Token::token_for_string("INPUT"), Some(Token::Input));
}

#[test]
fn test_token_for_builtins() {
    assert_eq!(Token::token_for_string("SIN"), Some(Token::BuiltInFn(BuiltInFunction::Sin)));
    assert_eq!(Token::token_for_string("COS"), Some(Token::BuiltInFn(BuiltInFunction::Cos)));
    assert_eq!(Token::token_for_string("SQRT"), Some(Token::BuiltInFn(BuiltInFunction::Sqrt)));
    assert_eq!(Token::token_for_string("ABS"), Some(Token::BuiltInFn(BuiltInFunction::Abs)));
    assert_eq!(Token::token_for_string("LEN"), Some(Token::BuiltInFn(BuiltInFunction::Len)));
    assert_eq!(Token::token_for_string("CHR"), Some(Token::BuiltInFn(BuiltInFunction::Chr)));
    assert_eq!(Token::token_for_string("ASC"), Some(Token::BuiltInFn(BuiltInFunction::Asc)));
}

#[test]
fn test_token_for_unknown() {
    assert_eq!(Token::token_for_string("UNKNOWN"), None);
    assert_eq!(Token::token_for_string(""), None);
    // Case sensitive
    assert_eq!(Token::token_for_string("goto"), None);
}

// ============================================
// Token Type Checks
// ============================================

#[test]
fn test_is_operator() {
    assert!(Token::Plus.is_operator());
    assert!(Token::Minus.is_operator());
    assert!(Token::Multiply.is_operator());
    assert!(Token::UMinus.is_operator());
    assert!(Token::Bang.is_operator());
    assert!(Token::Equals.is_operator());
    
    assert!(!Token::Variable("X".to_string()).is_operator());
    assert!(!Token::Number(5.0).is_operator());
    assert!(!Token::LParen.is_operator());
}

#[test]
fn test_is_comparison_operator() {
    assert!(Token::Equals.is_comparison_operator());
    assert!(Token::LessThan.is_comparison_operator());
    assert!(Token::NotEqual.is_comparison_operator());
    
    assert!(!Token::Plus.is_comparison_operator());
    assert!(!Token::UMinus.is_comparison_operator());
}

#[test]
fn test_is_unary_operator() {
    assert!(Token::UMinus.is_unary_operator());
    assert!(Token::Bang.is_unary_operator());
    assert!(!Token::Plus.is_unary_operator());
    assert!(!Token::Minus.is_unary_operator());
}

#[test]
fn test_is_binary_operator() {
    assert!(Token::Plus.is_binary_operator());
    assert!(Token::Minus.is_binary_operator());
    assert!(Token::Equals.is_binary_operator());
    
    // Unary operators are not binary
    assert!(!Token::UMinus.is_binary_operator());
    assert!(!Token::Bang.is_binary_operator());
}

#[test]
fn test_is_value() {
    assert!(Token::Variable("X".to_string()).is_value());
    assert!(Token::Number(5.0).is_value());
    assert!(Token::BString("hello".to_string()).is_value());
    assert!(Token::BuiltInFn(BuiltInFunction::Sin).is_value());
    
    assert!(!Token::Plus.is_value());
    assert!(!Token::Goto.is_value());
}

// ============================================
// Operator Precedence
// ============================================

#[test]
fn test_operator_precedence() {
    // Unary highest
    assert_eq!(Token::UMinus.operator_precedence(), Ok(12));
    assert_eq!(Token::Bang.operator_precedence(), Ok(12));
    
    // Multiplication
    assert_eq!(Token::Multiply.operator_precedence(), Ok(10));
    assert_eq!(Token::Divide.operator_precedence(), Ok(10));
    
    // Addition
    assert_eq!(Token::Plus.operator_precedence(), Ok(8));
    assert_eq!(Token::Minus.operator_precedence(), Ok(8));
    
    // Comparison lowest
    assert_eq!(Token::Equals.operator_precedence(), Ok(4));
    assert_eq!(Token::LessThan.operator_precedence(), Ok(4));
    
    // Non-operators error
    assert!(Token::Variable("X".to_string()).operator_precedence().is_err());
}

#[test]
fn test_precedence_order() {
    let unary = Token::UMinus.operator_precedence().unwrap();
    let mult = Token::Multiply.operator_precedence().unwrap();
    let add = Token::Plus.operator_precedence().unwrap();
    let comp = Token::Equals.operator_precedence().unwrap();
    
    assert!(unary > mult);
    assert!(mult > add);
    assert!(add > comp);
}

// ============================================
// Operator Associativity
// ============================================

#[test]
fn test_operator_associativity() {
    // Unary is right-associative
    assert_eq!(Token::UMinus.operator_associavity(), Ok(Associativity::Right));
    assert_eq!(Token::Bang.operator_associavity(), Ok(Associativity::Right));
    
    // Others are left-associative
    assert_eq!(Token::Plus.operator_associavity(), Ok(Associativity::Left));
    assert_eq!(Token::Multiply.operator_associavity(), Ok(Associativity::Left));
    assert_eq!(Token::Equals.operator_associavity(), Ok(Associativity::Left));
}

// ============================================
// BuiltInFunction Tests
// ============================================

#[test]
fn test_all_builtin_functions() {
    let _ = BuiltInFunction::Sin;
    let _ = BuiltInFunction::Cos;
    let _ = BuiltInFunction::Tan;
    let _ = BuiltInFunction::Asin;
    let _ = BuiltInFunction::Acos;
    let _ = BuiltInFunction::Atan;
    let _ = BuiltInFunction::Sqrt;
    let _ = BuiltInFunction::Abs;
    let _ = BuiltInFunction::Log;
    let _ = BuiltInFunction::Exp;
    let _ = BuiltInFunction::Floor;
    let _ = BuiltInFunction::Ceil;
    let _ = BuiltInFunction::Round;
    let _ = BuiltInFunction::Rand;
    let _ = BuiltInFunction::Num;
    let _ = BuiltInFunction::Str;
    let _ = BuiltInFunction::Len;
    let _ = BuiltInFunction::Chr;
    let _ = BuiltInFunction::Asc;
}

#[test]
fn test_builtin_clone_and_debug() {
    let func = BuiltInFunction::Cos;
    let cloned = func.clone();
    assert_eq!(func, cloned);
    
    let debug_str = format!("{:?}", func);
    assert!(debug_str.contains("Cos"));
}

// ============================================
// Token Equality and Clone
// ============================================

#[test]
fn test_token_equality() {
    assert_eq!(Token::Number(5.0), Token::Number(5.0));
    assert_ne!(Token::Number(5.0), Token::Number(3.0));
    
    assert_eq!(Token::Variable("X".to_string()), Token::Variable("X".to_string()));
    assert_ne!(Token::Variable("X".to_string()), Token::Variable("Y".to_string()));
    
    assert_eq!(Token::BString("hello".to_string()), Token::BString("hello".to_string()));
    assert_eq!(Token::BuiltInFn(BuiltInFunction::Sin), Token::BuiltInFn(BuiltInFunction::Sin));
}

#[test]
fn test_token_clone() {
    let token = Token::Number(5.0);
    assert_eq!(token.clone(), token);
}

// ============================================
// Associativity Tests
// ============================================

#[test]
fn test_associativity_clone() {
    assert_eq!(Associativity::Left.clone(), Associativity::Left);
    assert_eq!(Associativity::Right.clone(), Associativity::Right);
}
