use rbasic::value::RBasicValue;

// ============================================
// Negation (Unary Minus)
// ============================================

#[test]
fn test_negate() {
    assert_eq!(-RBasicValue::Number(5.0), Ok(RBasicValue::Number(-5.0)));
    assert_eq!(-RBasicValue::Number(-3.14), Ok(RBasicValue::Number(3.14)));
    assert!((-RBasicValue::String("hello".to_string())).is_err());
    assert!((-RBasicValue::Bool(true)).is_err());
}

// ============================================
// Logical NOT
// ============================================

#[test]
fn test_not() {
    assert_eq!(!RBasicValue::Bool(true), Ok(RBasicValue::Bool(false)));
    assert_eq!(!RBasicValue::Bool(false), Ok(RBasicValue::Bool(true)));
    assert!((!RBasicValue::Number(5.0)).is_err());
    assert!((!RBasicValue::String("hello".to_string())).is_err());
}

// ============================================
// Addition
// ============================================

#[test]
fn test_add() {
    // Number addition
    assert_eq!(RBasicValue::Number(3.0) + RBasicValue::Number(4.0), Ok(RBasicValue::Number(7.0)));
    
    // String concatenation
    assert_eq!(
        RBasicValue::String("Hello, ".to_string()) + RBasicValue::String("World!".to_string()),
        Ok(RBasicValue::String("Hello, World!".to_string()))
    );
    
    // Number-string coercion
    assert_eq!(RBasicValue::Number(5.0) + RBasicValue::String("3.0".to_string()), Ok(RBasicValue::Number(8.0)));
    assert_eq!(RBasicValue::String("10.0".to_string()) + RBasicValue::Number(2.5), Ok(RBasicValue::Number(12.5)));
    
    // Invalid combinations
    assert!((RBasicValue::Number(5.0) + RBasicValue::String("abc".to_string())).is_err());
    assert!((RBasicValue::Bool(true) + RBasicValue::Number(5.0)).is_err());
}

// ============================================
// Subtraction
// ============================================

#[test]
fn test_sub() {
    assert_eq!(RBasicValue::Number(10.0) - RBasicValue::Number(3.0), Ok(RBasicValue::Number(7.0)));
    assert_eq!(RBasicValue::Number(3.0) - RBasicValue::Number(10.0), Ok(RBasicValue::Number(-7.0)));
    
    // String-number coercion
    assert_eq!(RBasicValue::String("15.0".to_string()) - RBasicValue::Number(5.0), Ok(RBasicValue::Number(10.0)));
    
    // Invalid combinations
    assert!((RBasicValue::String("hello".to_string()) - RBasicValue::String("world".to_string())).is_err());
    assert!((RBasicValue::Number(5.0) - RBasicValue::Bool(true)).is_err());
}

// ============================================
// Multiplication
// ============================================

#[test]
fn test_mul() {
    assert_eq!(RBasicValue::Number(3.0) * RBasicValue::Number(4.0), Ok(RBasicValue::Number(12.0)));
    assert_eq!(RBasicValue::Number(999.0) * RBasicValue::Number(0.0), Ok(RBasicValue::Number(0.0)));
    assert_eq!(RBasicValue::Number(-2.0) * RBasicValue::Number(-3.0), Ok(RBasicValue::Number(6.0)));
    
    // String-number coercion
    assert_eq!(RBasicValue::Number(5.0) * RBasicValue::String("2.0".to_string()), Ok(RBasicValue::Number(10.0)));
    
    // Invalid combinations
    assert!((RBasicValue::String("hello".to_string()) * RBasicValue::String("world".to_string())).is_err());
    assert!((RBasicValue::Bool(true) * RBasicValue::Number(5.0)).is_err());
}

// ============================================
// Division
// ============================================

#[test]
fn test_div() {
    assert_eq!(RBasicValue::Number(10.0) / RBasicValue::Number(2.0), Ok(RBasicValue::Number(5.0)));
    
    // Division by zero produces infinity
    let result = RBasicValue::Number(10.0) / RBasicValue::Number(0.0);
    assert!(result.is_ok());
    match result.unwrap() {
        RBasicValue::Number(n) => assert!(n.is_infinite()),
        _ => panic!("Expected Number"),
    }
    
    // String-number coercion
    assert_eq!(RBasicValue::Number(15.0) / RBasicValue::String("3.0".to_string()), Ok(RBasicValue::Number(5.0)));
    
    // Invalid combinations
    assert!((RBasicValue::String("hello".to_string()) / RBasicValue::String("world".to_string())).is_err());
}

// ============================================
// Modulus
// ============================================

#[test]
fn test_rem() {
    assert_eq!(RBasicValue::Number(17.0) % RBasicValue::Number(5.0), Ok(RBasicValue::Number(2.0)));
    assert_eq!(RBasicValue::Number(10.0) % RBasicValue::Number(5.0), Ok(RBasicValue::Number(0.0)));
    
    // String-number coercion
    assert_eq!(RBasicValue::String("17.0".to_string()) % RBasicValue::Number(5.0), Ok(RBasicValue::Number(2.0)));
    
    // Invalid combinations
    assert!((RBasicValue::String("hello".to_string()) % RBasicValue::String("world".to_string())).is_err());
}

// ============================================
// Equality Comparison
// ============================================

#[test]
fn test_eq() {
    // Numbers
    assert_eq!(RBasicValue::Number(5.0).eq(&RBasicValue::Number(5.0)), Ok(true));
    assert_eq!(RBasicValue::Number(5.0).eq(&RBasicValue::Number(3.14)), Ok(false));
    
    // Strings
    assert_eq!(RBasicValue::String("hello".to_string()).eq(&RBasicValue::String("hello".to_string())), Ok(true));
    assert_eq!(RBasicValue::String("hello".to_string()).eq(&RBasicValue::String("world".to_string())), Ok(false));
    
    // Bools
    assert_eq!(RBasicValue::Bool(true).eq(&RBasicValue::Bool(true)), Ok(true));
    assert_eq!(RBasicValue::Bool(true).eq(&RBasicValue::Bool(false)), Ok(false));
    
    // Number-string coercion
    assert_eq!(RBasicValue::Number(5.0).eq(&RBasicValue::String("5.0".to_string())), Ok(true));
    assert_eq!(RBasicValue::String("3.14".to_string()).eq(&RBasicValue::Number(3.14)), Ok(true));
    
    // Invalid combinations
    assert!(RBasicValue::Number(5.0).eq(&RBasicValue::Bool(true)).is_err());
    assert!(RBasicValue::Number(5.0).eq(&RBasicValue::String("abc".to_string())).is_err());
}

#[test]
fn test_neq() {
    assert_eq!(RBasicValue::Number(5.0).neq(&RBasicValue::Number(3.0)), Ok(true));
    assert_eq!(RBasicValue::Number(5.0).neq(&RBasicValue::Number(5.0)), Ok(false));
    assert_eq!(RBasicValue::String("a".to_string()).neq(&RBasicValue::String("b".to_string())), Ok(true));
}

// ============================================
// Less Than Comparison
// ============================================

#[test]
fn test_lt() {
    // Numbers
    assert_eq!(RBasicValue::Number(3.0).lt(&RBasicValue::Number(5.0)), Ok(true));
    assert_eq!(RBasicValue::Number(7.0).lt(&RBasicValue::Number(5.0)), Ok(false));
    assert_eq!(RBasicValue::Number(5.0).lt(&RBasicValue::Number(5.0)), Ok(false));
    
    // Strings
    assert_eq!(RBasicValue::String("abc".to_string()).lt(&RBasicValue::String("def".to_string())), Ok(true));
    
    // Bools
    assert_eq!(RBasicValue::Bool(false).lt(&RBasicValue::Bool(true)), Ok(true));
    assert_eq!(RBasicValue::Bool(true).lt(&RBasicValue::Bool(false)), Ok(false));
    
    // Number-string coercion
    assert_eq!(RBasicValue::Number(3.0).lt(&RBasicValue::String("5.0".to_string())), Ok(true));
    
    // Invalid
    assert!(RBasicValue::Number(5.0).lt(&RBasicValue::Bool(true)).is_err());
}

// ============================================
// Greater Than Comparison
// ============================================

#[test]
fn test_gt() {
    assert_eq!(RBasicValue::Number(7.0).gt(&RBasicValue::Number(5.0)), Ok(true));
    assert_eq!(RBasicValue::Number(3.0).gt(&RBasicValue::Number(5.0)), Ok(false));
    assert_eq!(RBasicValue::String("xyz".to_string()).gt(&RBasicValue::String("abc".to_string())), Ok(true));
    assert_eq!(RBasicValue::Bool(true).gt(&RBasicValue::Bool(false)), Ok(true));
}

// ============================================
// Less/Greater Than or Equal
// ============================================

#[test]
fn test_lteq() {
    assert_eq!(RBasicValue::Number(3.0).lteq(&RBasicValue::Number(5.0)), Ok(true));
    assert_eq!(RBasicValue::Number(5.0).lteq(&RBasicValue::Number(5.0)), Ok(true));
    assert_eq!(RBasicValue::Number(7.0).lteq(&RBasicValue::Number(5.0)), Ok(false));
}

#[test]
fn test_gteq() {
    assert_eq!(RBasicValue::Number(7.0).gteq(&RBasicValue::Number(5.0)), Ok(true));
    assert_eq!(RBasicValue::Number(5.0).gteq(&RBasicValue::Number(5.0)), Ok(true));
    assert_eq!(RBasicValue::Number(3.0).gteq(&RBasicValue::Number(5.0)), Ok(false));
}

// ============================================
// Edge Cases
// ============================================

#[test]
fn test_nan_and_infinity() {
    let nan = RBasicValue::Number(f64::NAN);
    let five = RBasicValue::Number(5.0);
    
    // NaN comparisons are false
    assert_eq!(nan.eq(&five), Ok(false));
    assert_eq!(nan.lt(&five), Ok(false));
    assert_eq!(nan.gt(&five), Ok(false));
    
    // Infinity
    let inf = RBasicValue::Number(f64::INFINITY);
    assert_eq!(inf.gt(&five), Ok(true));
    assert_eq!(inf.lt(&five), Ok(false));
}
