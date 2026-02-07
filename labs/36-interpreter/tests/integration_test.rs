// Integration tests for Lab 36: Expression Interpreter
//
// Tests the full interpret pipeline (tokenize -> parse -> evaluate)
// as well as individual components.

use interpreter::{evaluate, interpret, tokenize, Expr, Operator, Parser, Token};

// ============================================================================
// BASIC ARITHMETIC
// ============================================================================

#[test]
fn test_addition() {
    assert_eq!(interpret("2 + 3").unwrap(), 5.0);
}

#[test]
fn test_subtraction() {
    assert_eq!(interpret("10 - 4").unwrap(), 6.0);
}

#[test]
fn test_multiplication() {
    assert_eq!(interpret("6 * 7").unwrap(), 42.0);
}

#[test]
fn test_division() {
    assert_eq!(interpret("20 / 5").unwrap(), 4.0);
}

#[test]
fn test_single_number() {
    assert_eq!(interpret("42").unwrap(), 42.0);
}

#[test]
fn test_decimal_number() {
    let result = interpret("3.14").unwrap();
    assert!((result - 3.14).abs() < f64::EPSILON);
}

// ============================================================================
// OPERATOR PRECEDENCE
// ============================================================================

#[test]
fn test_multiplication_before_addition() {
    // 2 + 3 * 4 = 2 + 12 = 14 (not 20)
    assert_eq!(interpret("2 + 3 * 4").unwrap(), 14.0);
}

#[test]
fn test_multiplication_before_subtraction() {
    // 10 - 2 * 3 = 10 - 6 = 4 (not 24)
    assert_eq!(interpret("10 - 2 * 3").unwrap(), 4.0);
}

#[test]
fn test_division_before_addition() {
    // 10 + 6 / 3 = 10 + 2 = 12 (not ~5.33)
    assert_eq!(interpret("10 + 6 / 3").unwrap(), 12.0);
}

#[test]
fn test_mixed_precedence() {
    // 100 * 2 + 12 = 200 + 12 = 212
    assert_eq!(interpret("100 * 2 + 12").unwrap(), 212.0);
}

#[test]
fn test_add_then_multiply() {
    // 10 + 2 * 6 = 10 + 12 = 22
    assert_eq!(interpret("10 + 2 * 6").unwrap(), 22.0);
}

// ============================================================================
// PARENTHESES
// ============================================================================

#[test]
fn test_parentheses_override_precedence() {
    // (2 + 3) * 4 = 5 * 4 = 20
    assert_eq!(interpret("(2 + 3) * 4").unwrap(), 20.0);
}

#[test]
fn test_parentheses_around_multiplication() {
    // 100 * (2 + 12) = 100 * 14 = 1400
    assert_eq!(interpret("100 * (2 + 12)").unwrap(), 1400.0);
}

#[test]
fn test_nested_parentheses() {
    // ((15 / (7 - (1 + 1))) * 3) - (2 + (1 + 1))
    // = ((15 / (7 - 2)) * 3) - (2 + 2)
    // = ((15 / 5) * 3) - 4
    // = (3 * 3) - 4
    // = 9 - 4 = 5
    assert_eq!(
        interpret("((15 / (7 - (1 + 1))) * 3) - (2 + (1 + 1))").unwrap(),
        5.0
    );
}

#[test]
fn test_parenthesized_subtraction_times_addition() {
    // (5 - 3) * (7 + 2) = 2 * 9 = 18
    assert_eq!(interpret("(5 - 3) * (7 + 2)").unwrap(), 18.0);
}

// ============================================================================
// LEFT ASSOCIATIVITY
// ============================================================================

#[test]
fn test_left_associativity_addition() {
    // 1 + 2 + 3 + 4 + 5 = 15
    assert_eq!(interpret("1 + 2 + 3 + 4 + 5").unwrap(), 15.0);
}

#[test]
fn test_left_associativity_subtraction() {
    // 10 - 5 - 2 = (10 - 5) - 2 = 3
    assert_eq!(interpret("10 - 5 - 2").unwrap(), 3.0);
}

#[test]
fn test_left_associativity_multiplication() {
    // 2 * 3 * 4 = (2 * 3) * 4 = 24
    assert_eq!(interpret("2 * 3 * 4").unwrap(), 24.0);
}

#[test]
fn test_left_associativity_division() {
    // 100 / 10 / 2 = (100 / 10) / 2 = 5
    assert_eq!(interpret("100 / 10 / 2").unwrap(), 5.0);
}

// ============================================================================
// ERROR CASES
// ============================================================================

#[test]
fn test_empty_expression() {
    let result = interpret("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Empty expression"));
}

#[test]
fn test_missing_right_operand() {
    // "2 +" -- after parsing 2 and consuming +, no term to parse
    let result = interpret("2 +");
    assert!(result.is_err());
}

#[test]
fn test_missing_left_operand() {
    // "* 3" -- factor encounters * which is not a number or paren
    let result = interpret("* 3");
    assert!(result.is_err());
}

#[test]
fn test_double_operator() {
    // "2 + + 3" -- after first +, factor encounters second +
    let result = interpret("2 + + 3");
    assert!(result.is_err());
}

#[test]
fn test_unclosed_parenthesis() {
    let result = interpret("(2 + 3");
    assert!(result.is_err());
}

#[test]
fn test_extra_closing_parenthesis() {
    let result = interpret("2 + 3)");
    assert!(result.is_err());
}

#[test]
fn test_division_by_zero() {
    let result = interpret("10 / 0");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Division by zero"));
}

#[test]
fn test_division_by_zero_in_subexpression() {
    let result = interpret("5 + 10 / 0");
    assert!(result.is_err());
}

// ============================================================================
// TOKENIZER TESTS
// ============================================================================

#[test]
fn test_tokenize_empty() {
    let tokens = tokenize("");
    assert!(tokens.is_empty());
}

#[test]
fn test_tokenize_whitespace_only() {
    let tokens = tokenize("   \t\n  ");
    assert!(tokens.is_empty());
}

#[test]
fn test_tokenize_number() {
    let tokens = tokenize("42");
    assert_eq!(tokens, vec![Token::Number(42.0)]);
}

#[test]
fn test_tokenize_decimal_number() {
    let tokens = tokenize("3.14");
    assert_eq!(tokens, vec![Token::Number(3.14)]);
}

#[test]
fn test_tokenize_all_operators() {
    let tokens = tokenize("+ - * /");
    assert_eq!(
        tokens,
        vec![Token::Plus, Token::Minus, Token::Star, Token::Slash]
    );
}

#[test]
fn test_tokenize_parentheses() {
    let tokens = tokenize("()");
    assert_eq!(tokens, vec![Token::LeftParen, Token::RightParen]);
}

#[test]
fn test_tokenize_complex_expression() {
    let tokens = tokenize("(2 + 3) * 4");
    assert_eq!(
        tokens,
        vec![
            Token::LeftParen,
            Token::Number(2.0),
            Token::Plus,
            Token::Number(3.0),
            Token::RightParen,
            Token::Star,
            Token::Number(4.0),
        ]
    );
}

#[test]
fn test_tokenize_no_spaces() {
    let tokens = tokenize("2+3*4");
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Token::Number(2.0));
    assert_eq!(tokens[1], Token::Plus);
    assert_eq!(tokens[2], Token::Number(3.0));
    assert_eq!(tokens[3], Token::Star);
    assert_eq!(tokens[4], Token::Number(4.0));
}

// ============================================================================
// PARSER TESTS
// ============================================================================

#[test]
fn test_parser_single_number() {
    let tokens = tokenize("7");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    match ast {
        Expr::Number(n) => assert_eq!(n, 7.0),
        _ => panic!("Expected Expr::Number"),
    }
}

#[test]
fn test_parser_binary_addition() {
    let tokens = tokenize("2 + 3");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    match ast {
        Expr::BinOp { op, .. } => assert_eq!(op, Operator::Add),
        _ => panic!("Expected Expr::BinOp"),
    }
}

#[test]
fn test_parser_precedence_structure() {
    // 2 + 3 * 4 should parse as Add(2, Mul(3, 4))
    let tokens = tokenize("2 + 3 * 4");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    match &ast {
        Expr::BinOp {
            left,
            op: Operator::Add,
            right,
        } => {
            // Left should be Number(2)
            match left.as_ref() {
                Expr::Number(n) => assert_eq!(*n, 2.0),
                _ => panic!("Expected left to be Number(2)"),
            }
            // Right should be BinOp(3, Mul, 4)
            match right.as_ref() {
                Expr::BinOp {
                    op: Operator::Multiply,
                    ..
                } => {}
                _ => panic!("Expected right to be Multiply"),
            }
        }
        _ => panic!("Expected top-level Add"),
    }
}

#[test]
fn test_parser_empty_tokens() {
    let parser_result = Parser::new(vec![]).parse();
    assert!(parser_result.is_err());
}

// ============================================================================
// EVALUATOR TESTS
// ============================================================================

#[test]
fn test_evaluate_literal() {
    let expr = Expr::Number(99.0);
    assert_eq!(evaluate(&expr).unwrap(), 99.0);
}

#[test]
fn test_evaluate_addition() {
    let expr = Expr::BinOp {
        left: Box::new(Expr::Number(10.0)),
        op: Operator::Add,
        right: Box::new(Expr::Number(20.0)),
    };
    assert_eq!(evaluate(&expr).unwrap(), 30.0);
}

#[test]
fn test_evaluate_subtraction() {
    let expr = Expr::BinOp {
        left: Box::new(Expr::Number(50.0)),
        op: Operator::Subtract,
        right: Box::new(Expr::Number(25.0)),
    };
    assert_eq!(evaluate(&expr).unwrap(), 25.0);
}

#[test]
fn test_evaluate_multiplication() {
    let expr = Expr::BinOp {
        left: Box::new(Expr::Number(7.0)),
        op: Operator::Multiply,
        right: Box::new(Expr::Number(8.0)),
    };
    assert_eq!(evaluate(&expr).unwrap(), 56.0);
}

#[test]
fn test_evaluate_division() {
    let expr = Expr::BinOp {
        left: Box::new(Expr::Number(100.0)),
        op: Operator::Divide,
        right: Box::new(Expr::Number(4.0)),
    };
    assert_eq!(evaluate(&expr).unwrap(), 25.0);
}

#[test]
fn test_evaluate_nested() {
    // (3 + 4) * 2 = 14
    let expr = Expr::BinOp {
        left: Box::new(Expr::BinOp {
            left: Box::new(Expr::Number(3.0)),
            op: Operator::Add,
            right: Box::new(Expr::Number(4.0)),
        }),
        op: Operator::Multiply,
        right: Box::new(Expr::Number(2.0)),
    };
    assert_eq!(evaluate(&expr).unwrap(), 14.0);
}

#[test]
fn test_evaluate_division_by_zero_direct() {
    let expr = Expr::BinOp {
        left: Box::new(Expr::Number(1.0)),
        op: Operator::Divide,
        right: Box::new(Expr::Number(0.0)),
    };
    assert!(evaluate(&expr).is_err());
}

// ============================================================================
// END-TO-END COMPLEX EXPRESSIONS
// ============================================================================

#[test]
fn test_complex_nested_expression() {
    // ((15 / (7 - (1 + 1))) * 3) - (2 + (1 + 1)) = 5
    let result = interpret("((15 / (7 - (1 + 1))) * 3) - (2 + (1 + 1))").unwrap();
    assert_eq!(result, 5.0);
}

#[test]
fn test_large_chain_of_additions() {
    // 1 + 2 + 3 + ... + 10 = 55
    let result = interpret("1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10").unwrap();
    assert_eq!(result, 55.0);
}

#[test]
fn test_mixed_operations() {
    // 2 * 3 + 4 * 5 = 6 + 20 = 26
    assert_eq!(interpret("2 * 3 + 4 * 5").unwrap(), 26.0);
}

#[test]
fn test_deeply_nested_parentheses() {
    // (((1 + 2))) = 3
    assert_eq!(interpret("(((1 + 2)))").unwrap(), 3.0);
}

#[test]
fn test_zero_result() {
    assert_eq!(interpret("5 - 5").unwrap(), 0.0);
}

#[test]
fn test_negative_result() {
    assert_eq!(interpret("3 - 7").unwrap(), -4.0);
}

#[test]
fn test_fractional_result() {
    let result = interpret("1 / 3").unwrap();
    assert!((result - 1.0 / 3.0).abs() < 1e-10);
}

#[test]
fn test_large_numbers() {
    assert_eq!(interpret("1000000 * 1000000").unwrap(), 1e12);
}

#[test]
fn test_decimal_arithmetic() {
    let result = interpret("0.1 + 0.2").unwrap();
    assert!((result - 0.3).abs() < 1e-10);
}
