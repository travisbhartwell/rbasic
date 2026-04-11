use rbasic::value::RBasicValue;
use rbasic::evaluator;
use rbasic::lexer;

// ============================================
// BUG #1: Boolean lt comparison uses == instead of <
// ============================================
#[test]
fn test_boolean_lt_comparison() {
    // false < true should be true
    let false_val = RBasicValue::Bool(false);
    let true_val = RBasicValue::Bool(true);
    
    let result = false_val.lt(&true_val);
    assert!(result.is_ok());
    // This should be true (false < true is true)
    assert_eq!(result.unwrap(), true, "false < true should be true");
}

#[test]
fn test_boolean_lt_false_true() {
    // true < false should be false
    let false_val = RBasicValue::Bool(false);
    let true_val = RBasicValue::Bool(true);
    
    let result = true_val.lt(&false_val);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false, "true < false should be false");
}

#[test]
fn test_boolean_lt_false_false() {
    // false < false should be false
    let false_val = RBasicValue::Bool(false);
    
    let result = false_val.lt(&false_val);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false, "false < false should be false");
}

#[test]
fn test_boolean_lt_true_true() {
    // true < true should be false
    let true_val = RBasicValue::Bool(true);
    
    let result = true_val.lt(&true_val);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false, "true < true should be false");
}

// ============================================
// BUG #3: lteq and gteq logic verification
// ============================================
#[test]
fn test_lteq_correctness() {
    let a = RBasicValue::Number(5.0);
    let b = RBasicValue::Number(5.0);
    let c = RBasicValue::Number(10.0);
    
    // 5 <= 5 should be true
    assert_eq!(a.lteq(&b).unwrap(), true);
    // 5 <= 10 should be true
    assert_eq!(a.lteq(&c).unwrap(), true);
    // 10 <= 5 should be false
    assert_eq!(c.lteq(&a).unwrap(), false);
}

#[test]
fn test_gteq_correctness() {
    let a = RBasicValue::Number(5.0);
    let b = RBasicValue::Number(5.0);
    let c = RBasicValue::Number(10.0);
    
    // 5 >= 5 should be true
    assert_eq!(a.gteq(&b).unwrap(), true);
    // 10 >= 5 should be true
    assert_eq!(c.gteq(&a).unwrap(), true);
    // 5 >= 10 should be false
    assert_eq!(a.gteq(&c).unwrap(), false);
}

// ============================================
// BUG #6: unreachable!() panic in shunting-yard
// ============================================
#[test]
fn test_bad_token_in_expression() {
    // This has GOTO in the middle of an expression which shouldn't happen
    // But if it does, it should error gracefully, not panic
    let code_lines = vec![
        lexer::tokenize_line("10 LET X = 5 + GOTO 100", false)
    ];
    
    // If tokenize fails, that's acceptable (though could be better)
    if code_lines[0].is_ok() {
        let code_lines = vec![code_lines[0].clone().unwrap()];
        let result = std::panic::catch_unwind(|| {
            evaluator::evaluate(code_lines)
        });
        
        assert!(result.is_ok(), "Bad token in expression should not cause panic");
    }
}

// ============================================
// BUG #10: Assertion panic on empty expression
// ============================================
#[test]
fn test_empty_print_expression() {
    // PRINT with no expression might trigger the assert
    let code_lines = vec![
        lexer::tokenize_line("10 PRINT", false).unwrap()
    ];
    
    let result = std::panic::catch_unwind(|| {
        evaluator::evaluate(code_lines)
    });
    
    // Should error gracefully, not panic
    assert!(result.is_ok(), "Empty PRINT expression should not panic");
}

// ============================================
// BUG #12: String with quotes handling
// ============================================
#[test]
fn test_string_with_escaped_quotes() {
    // BASIC typically uses "" for escaped quotes
    let result = lexer::tokenize_line(r#"10 PRINT "Hello ""World""""#, false);
    // This might fail or produce wrong results
    println!("String with quotes result: {:?}", result);
}

#[test]
fn test_unclosed_string() {
    // Unclosed string literal
    let result = lexer::tokenize_line(r#"10 PRINT "Hello world"#, false);
    // This should error but currently consumes entire rest of line
    println!("Unclosed string result: {:?}", result);
}

// ============================================
// Integration test: test2.bas comparison operators
// ============================================
#[test]
fn test_comparison_operators_integration() {
    let program = r#"10 LET X = 50
20 PRINT X > 5
30 PRINT X < 5
40 PRINT X = 5
50 PRINT X <> 5
60 PRINT X <= 5
70 PRINT X >= 5
"#;
    
    let code_lines: Vec<lexer::LineOfCode> = program
        .lines()
        .enumerate()
        .filter_map(|(_, line)| {
            let result = lexer::tokenize_line(line, false);
            if result.is_ok() {
                Some(result.unwrap())
            } else {
                None
            }
        })
        .collect();
    
    let result = evaluator::evaluate(code_lines);
    assert!(result.is_ok(), "Comparison operators test failed: {:?}", result);
}
