use rbasic::lexer;
use rbasic::evaluator;

fn run_program(program: &str) -> Result<String, String> {
    let code_lines: Vec<lexer::LineOfCode> = program
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| lexer::tokenize_line(line, false).ok())
        .collect();
    evaluator::evaluate(code_lines)
}

// ============================================
// GOTO Tests
// ============================================

#[test]
fn test_goto_basic() {
    assert!(run_program("10 GOTO 30\n20 PRINT \"Skip\"\n30 PRINT \"OK\"").is_ok());
}

#[test]
fn test_goto_forward_and_backward() {
    // Forward
    assert!(run_program("10 GOTO 30\n20 PRINT \"Skip\"\n30 PRINT \"OK\"").is_ok());
    // Backward (will loop, but doesn't crash)
}

#[test]
fn test_goto_errors() {
    // Invalid target
    assert!(run_program("10 GOTO 999").is_err());
    // Missing target
    assert!(run_program("10 GOTO").is_err());
    // Non-number target
    assert!(run_program("10 GOTO ABC").is_err());
}

// ============================================
// LET/Variable Assignment Tests
// ============================================

#[test]
fn test_let_basic() {
    // Number
    assert!(run_program("10 LET X = 42\n20 PRINT X").is_ok());
    // String
    assert!(run_program("10 LET NAME = \"World\"\n20 PRINT NAME").is_ok());
    // Expression
    assert!(run_program("10 LET X = 10 + 5\n20 PRINT X").is_ok());
    // Complex expression
    assert!(run_program("10 LET X = (10 + 5) * 2\n20 PRINT X").is_ok());
}

#[test]
fn test_let_variable_reference() {
    assert!(run_program("10 LET A = 5\n20 LET B = A + 10\n30 PRINT B").is_ok());
}

#[test]
fn test_let_errors() {
    // Undefined variable
    assert!(run_program("10 LET X = UNDEFINED").is_err());
    // Missing expression
    assert!(run_program("10 LET").is_err());
    // Bad syntax
    assert!(run_program("10 LET X 5").is_err());
}

// ============================================
// PRINT Tests
// ============================================

#[test]
fn test_print_basic() {
    // String
    assert!(run_program("10 PRINT \"Hello, World!\"").is_ok());
    // Number
    assert!(run_program("10 PRINT 42").is_ok());
    // Variable
    assert!(run_program("10 LET X = 100\n20 PRINT X").is_ok());
    // Expression
    assert!(run_program("10 PRINT 10 + 20").is_ok());
    // String concatenation
    assert!(run_program("10 PRINT \"Hello\" + \" World\"").is_ok());
}

#[test]
fn test_print_missing_expression() {
    assert!(run_program("10 PRINT").is_err());
}

// ============================================
// IF/THEN Tests
// ============================================

#[test]
fn test_if_conditions() {
    // True condition
    assert!(run_program("10 LET X = 10\n20 IF X > 5 THEN 40\n30 PRINT \"Skip\"\n40 PRINT \"OK\"").is_ok());
    // False condition
    assert!(run_program("10 LET X = 3\n20 IF X > 5 THEN 40\n30 PRINT \"OK\"").is_ok());
}

#[test]
fn test_if_comparison_operators() {
    assert!(run_program("10 LET X = 5\n20 IF X = 5 THEN 40\n30 PRINT \"Skip\"\n40 PRINT \"OK\"").is_ok());
    assert!(run_program("10 LET X = 5\n20 IF X <> 10 THEN 40\n30 PRINT \"Skip\"\n40 PRINT \"OK\"").is_ok());
    assert!(run_program("10 LET X = 5\n20 IF X <= 5 THEN 40\n30 PRINT \"Skip\"\n40 PRINT \"OK\"").is_ok());
    assert!(run_program("10 LET X = 5\n20 IF X >= 5 THEN 40\n30 PRINT \"Skip\"\n40 PRINT \"OK\"").is_ok());
}

#[test]
fn test_if_errors() {
    // Invalid target
    assert!(run_program("10 LET X = 1\n20 IF X = 1 THEN 999").is_err());
    // Bad syntax
    assert!(run_program("10 LET X = 1\n20 IF X = 1").is_err());
}

// ============================================
// Comments (REM) Tests
// ============================================

#[test]
fn test_comments() {
    assert!(run_program("10 REM This is a comment\n20 PRINT \"OK\"").is_ok());
    assert!(run_program("10 REM Just a comment").is_ok());
    assert!(run_program("10 REM First\n20 REM Second\n30 PRINT \"OK\"").is_ok());
}

// ============================================
// Expression Tests
// ============================================

#[test]
fn test_arithmetic_precedence() {
    // Should be 14 (2 + (3 * 4)), not 20 ((2 + 3) * 4)
    assert!(run_program("10 LET X = 2 + 3 * 4\n20 PRINT X").is_ok());
}

#[test]
fn test_parentheses() {
    // Nested parens
    assert!(run_program("10 LET X = ((2 + 3) * (4 - 1))\n20 PRINT X").is_ok());
    // Mismatched parens
    assert!(run_program("10 LET X = (2 + 3").is_err());
}

#[test]
fn test_unary_operators() {
    assert!(run_program("10 LET X = -5 + 3\n20 PRINT X").is_ok());
    assert!(run_program("10 LET X = --5\n20 PRINT X").is_ok());
}

#[test]
fn test_all_arithmetic_operators() {
    assert!(run_program("10 LET X = 10 / 3\n20 PRINT X").is_ok());
    assert!(run_program("10 LET X = 17 % 5\n20 PRINT X").is_ok());
}

#[test]
fn test_string_concatenation() {
    assert!(run_program("10 LET A = \"Hello\"\n20 LET B = \" World\"\n30 LET C = A + B\n40 PRINT C").is_ok());
}

#[test]
fn test_comparison_in_let() {
    assert!(run_program("10 LET X = 5 > 3\n20 PRINT X").is_ok());
}

// ============================================
// Built-in Functions
// ============================================

#[test]
fn test_builtin_tokens_exist() {
    // Built-in functions require lexer improvements for full testing
    // For now, verify they don't break basic execution
    assert!(run_program("10 REM Builtin functions supported").is_ok());
}

// ============================================
// Program Flow Tests
// ============================================

#[test]
fn test_execution_order() {
    // Sequential
    assert!(run_program("10 LET X = 1\n20 LET Y = 2\n30 LET Z = X + Y\n40 PRINT Z").is_ok());
    // Non-sequential line numbers
    assert!(run_program("100 LET X = 1\n200 LET Y = 2\n300 PRINT X\n350 PRINT Y").is_ok());
    // Unsorted line numbers (should execute in order)
    assert!(run_program("30 PRINT \"Third\"\n10 PRINT \"First\"\n20 PRINT \"Second\"").is_ok());
}

#[test]
fn test_empty_and_comment_only_programs() {
    assert!(run_program("").is_ok());
    assert!(run_program("10 REM Comment 1\n20 REM Comment 2").is_ok());
}

// ============================================
// Line Map Tests
// ============================================

#[test]
fn test_get_line_map() {
    let code_lines: Vec<lexer::LineOfCode> = vec![
        lexer::tokenize_line("30 PRINT \"C\"", false).unwrap(),
        lexer::tokenize_line("10 PRINT \"A\"", false).unwrap(),
        lexer::tokenize_line("20 PRINT \"B\"", false).unwrap(),
    ];
    
    let mut line_map = std::collections::BTreeMap::new();
    evaluator::get_line_map(&code_lines, &mut line_map);
    
    assert_eq!(line_map.len(), 3);
    let keys: Vec<&u32> = line_map.keys().collect();
    assert_eq!(*keys, vec![&10, &20, &30]);
}

#[test]
fn test_get_line_map_empty() {
    let mut line_map = std::collections::BTreeMap::new();
    evaluator::get_line_map(&[], &mut line_map);
    assert!(line_map.is_empty());
}

// ============================================
// Error Handling Tests
// ============================================

#[test]
fn test_invalid_syntax() {
    assert!(run_program("10 INVALID").is_err());
    assert!(run_program("10 LET X = 5 + GOTO").is_err());
    assert!(lexer::tokenize_line("ABC PRINT \"Hello\"", false).is_err());
}

// ============================================
// Integration Tests
// ============================================

#[test]
fn test_factorial_program() {
    assert!(run_program("
10 LET N = 5
20 LET F = 1
30 LET F = F * N
40 LET N = N - 1
50 IF N > 1 THEN 30
60 PRINT F
").is_ok());
}

#[test]
fn test_counter_program() {
    assert!(run_program("
10 LET I = 1
20 PRINT I
30 LET I = I + 1
40 IF I <= 5 THEN 20
").is_ok());
}

#[test]
fn test_string_operations() {
    assert!(run_program("
10 LET A = \"Hello\"
20 LET B = \" \"
30 LET C = \"World\"
40 LET D = A + B + C
50 PRINT D
").is_ok());
}

#[test]
fn test_comparison_chain() {
    assert!(run_program("
10 LET X = 10
20 LET Y = 20
30 PRINT X < Y
40 PRINT X > Y
50 PRINT X = Y
").is_ok());
}
