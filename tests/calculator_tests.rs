use rustcalculator::calculator;

#[test]
pub fn test_sum() {
    let result = calculator::Calculator::resolve("2+2".to_owned());

    assert_eq!(result, Some(4.0))
}

#[test]
pub fn test_sum_with_expression() {
    let result = calculator::Calculator::resolve("(2+2) + 5 + (3+3)".to_owned());

    assert_eq!(result, Some(15.0))
}

#[test]
pub fn test_sub() {
    let result = calculator::Calculator::resolve("2-2".to_owned());

    assert_eq!(result, Some(0.0))
}

#[test]
pub fn test_sub_with_expression() {
    let result = calculator::Calculator::resolve("(8-2) - 1 - (3-1)".to_owned());

    assert_eq!(result, Some(3.0))
}

#[test]
pub fn test_sub_with_expression_new() {
    let result = calculator::Calculator::resolve("4 + 4 * 2 / ( 1 - 5 )".to_owned());

    assert_eq!(result, Some(2.0))
}

#[test]
pub fn test_expression_with_float() {
    let result = calculator::Calculator::resolve("2.5 + 3.5".to_owned()).unwrap();
    println!("{:?}", result);
    assert_eq!(result, 6.0)
}