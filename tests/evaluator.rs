extern crate rbasic;

use rbasic::lexer::*;
use rbasic::evaluator::*;

#[test]
fn eval_goto_invalid_target_line_number() {
    let result = tokenize_line("10 GOTO 5");
    assert!(result.is_ok());
    let code_line = result.unwrap();
    let eval_result = evaluate(vec![code_line]).err();
    assert_eq!(eval_result,
               Some("At LineNumber(10), 8 invalid target line for GOTO".to_string()));
}

#[test]
fn eval_goto_invalid_line_number() {
    let result = tokenize_line("10 GOTO A");
    assert!(result.is_ok());
    let code_line = result.unwrap();
    let eval_result = evaluate(vec![code_line]).err();
    assert_eq!(eval_result,
               Some("At LineNumber(10), 8 GOTO must be followed by valid line number".to_string()));
}

#[test]
fn eval_goto_no_line_number() {
    let result = tokenize_line("10 GOTO");
    assert!(result.is_ok());
    let code_line = result.unwrap();
    let eval_result = evaluate(vec![code_line]).err();
    assert_eq!(eval_result,
               Some("At LineNumber(10), 7 GOTO must be followed by a line number".to_string()));
}

#[test]
fn eval_input_no_variable() {
    let result = tokenize_line("10 INPUT");
    assert!(result.is_ok());
    let code_line = result.unwrap();
    let eval_result = evaluate(vec![code_line]).err();
    assert_eq!(eval_result,
               Some("At LineNumber(10), 8 INPUT must be followed by a variable name".to_string()));
}
