use rustcalculator::calculator;
use rustcalculator::calculator::{Token, Operator};

#[test]
fn test_parser() {
    let tokens_result = crate::calculator::Calculator::parse("1 + 2 * (3 - 4)");

    let tokens = tokens_result.unwrap();

    let expected_tokens = vec![
        Token::Number(1.0), Token::Operator(Operator::Add),
        Token::Number(2.0), Token::Operator(Operator::Mul),
        Token::Bracket('('), Token::Number(3.0), 
        Token::Operator(Operator::Sub), Token::Number(4.0), Token::Bracket(')')
    ];

    assert_eq!(expected_tokens, tokens);    
}

#[test]
fn test_parse_decimal() {
    let tokens_result = crate::calculator::Calculator::parse("2.5");

    let tokens = tokens_result.unwrap();

    let expected_tokens = vec![
        Token::Number(2.5)
    ];

    assert_eq!(expected_tokens, tokens)
}

// EXPRESSION: 2.5 
// TOKEN | ACTION                        | QUEUE         | STACK
#[test]
fn test_parse_with_decimals() {
    let tokens_result = crate::calculator::Calculator::parse("2.5 + 3.5");

    let tokens = tokens_result.unwrap();

    let expected_tokens = vec![
        Token::Number(2.5), Token::Operator(Operator::Add), Token::Number(3.5)
    ];

    assert_eq!(expected_tokens, tokens)
}

// EXPRESSION: 1 + 2 * (3 - 4)
// TOKEN | ACTION                        | QUEUE         | STACK
// 1     | Add to queue                  | 1             |
// +     | Add to stack                  | 1             | +
// 2     | Add to queue                  | 1 2           | +
// *     | Add to stack                  | 1 2           | * +
// (     | Add to stack                  | 1 2           | ( * +
// 3     | Add to queue                  | 1 2 3         | ( * +
// -     | Add to stack                  | 1 2 3         | - ( * +
// 4     | Add to queue                  | 1 2 3 4       | - ( * +
// )     | Pop stack, pop stack to queue | 1 2 3 4 -     | * +
// end   | Pop entire stack to output    | 1 2 3 4 - * + |
// 
// Entire queue output: 1 2 3 4 - * +
#[test]
fn test_expression_queue() {
    let tokens_result = calculator::Calculator::parse("1 + 2 * (3 - 4)");

    let expression_queue = calculator::Calculator::expression(tokens_result.unwrap());

    let expected_queue: Vec<Token> = vec![
        Token::Number(1.0), Token::Number(2.0), Token::Number(3.0), Token::Number(4.0),
        Token::Operator(Operator::Sub), Token::Operator(Operator::Mul), Token::Operator(Operator::Add)
    ];

    assert_eq!(expression_queue, expected_queue)
}

// EXPRESSION: (5 * 3) + (8 - 2) / 4
// TOKEN | ACTION                        | QUEUE             | STACK
// (     | Add to stack                  |                   | (
// 5     | Add to queue                  | 5                 | (
// *     | Add to stack                  | 5                 | * (
// 3     | Add to queue                  | 5 3               | * (
// )     | Pop stack to queue            | 5 3 *             |
// +     | Add to stack                  | 5 3 *             | +
// (     | Add to stack                  | 5 3 *             | ( +
// 8     | Add to queue                  | 5 3 * 8           | ( +
// -     | Add to stack                  | 5 3 * 8           | - ( +
// 2     | Add to queue                  | 5 3 * 8 2         | - ( +
// )     | Pop stack to queue            | 5 3 * 8 2 -       | +
// /     | Add to stack                  | 5 3 * 8 2 -       | / +
// 4     | Add to queue                  | 5 3 * 8 2 - 4     | / +
// end   | Pop entire stack to output    | 5 3 * 8 2 - 4 / + |
// 
// Entire queue output: 5 3 * 8 2 - 4 / +
#[test]
fn test_second_expression_queue() {
    let tokens_result = calculator::Calculator::parse("(5 * 3) + (8 - 2) / 4");

    let expression_queue = calculator::Calculator::expression(tokens_result.unwrap());

    let expected_queue: Vec<Token> = vec![
        Token::Number(5.0), Token::Number(3.0), Token::Operator(Operator::Mul), Token::Number(8.0), Token::Number(2.0),
        Token::Operator(Operator::Sub), Token::Number(4.0), Token::Operator(Operator::Div), Token::Operator(Operator::Add)
    ];

    assert_eq!(expression_queue, expected_queue)
}

// EXPRESSION: 4 + 4 * 2 / ( 1 - 5 )
//
// TOKEN | ACTION                        | QUEUE             | STACK
// 4     | Add to queue                  | 4                 |
// +     | Add to stack                  | 4                 | +
// 4     | Add to queue                  | 4 4               | +
// *     | Add to stack                  | 4 4               | * +
// 2     | Add to queue                  | 4 4 2             | * +
// /     | * to queue and / to stack     | 4 4 2 *           | / +
// (     | Add to stack                  | 4 4 2 *           | ( / +
// 1     | Add to queue                  | 4 4 2 * 1         | ( / +
// -     | Add to stack                  | 4 4 2 * 1         | - ( / +
// 5     | Add to queue                  | 4 4 2 * 1 5       | - ( / +
// )     | Pop stack, pop stack to queue | 4 4 2 * 1 5 -     | / +
// end   | Pop entire stack to output    | 4 4 2 * 1 5 - / + |
//
// Entire queue output: 4 4 2 * 1 5 - / +	
#[test]
fn test_third_expression_queue() {
    let tokens_result = calculator::Calculator::parse("4 + 4 * 2 / ( 1 - 5 )");

    let expression_queue = calculator::Calculator::expression(tokens_result.unwrap());

    let expected_queue: Vec<Token> = vec![
        Token::Number(4.0), Token::Number(4.0), Token::Number(2.0), Token::Operator(Operator::Mul), Token::Number(1.0),
        Token::Number(5.0), Token::Operator(Operator::Sub), Token::Operator(Operator::Div), Token::Operator(Operator::Add)
    ];

    assert_eq!(expression_queue, expected_queue)
}