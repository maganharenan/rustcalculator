// #[path = "../src/calculator/calculator_button.rs"]
pub mod calculator_button;

// #[path = "../src/calculator/screen_settings.rs"]
pub mod screen_settings;
use screen_settings::{ROW_SPACE, COLUMN_SPACE};

use std::char;

use iced::executor;
use iced::{Application, Command, Theme};
use iced::widget::{column, container, row, text, vertical_space};
use iced::alignment;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn value(&self) -> u16 {
        match self {
            Self::Add => 0,
            Self::Sub => 0,
            Self::Mul => 5,
            Self::Div => 5,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Token {
    Number(f32),
    Operator(Operator),
    Bracket(char),
}

pub struct Calculator {
    expression: String,
    result: Option<String>
}

#[derive(Debug)]
pub enum Error {
    BadToken(char),
    MismatchedParens,
}

impl Calculator {

    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref();
        let chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parens = Vec::new(); 
        let mut is_calculating_decimals: bool = false;
        let mut decimal_place: f32 = 0.1;

        for c in chars {
            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        if is_calculating_decimals {
                            *n += ((c as i32) as f32 - 48.0) * decimal_place;
                            decimal_place *= 0.1;
                        } else {
                            *n = *n * 10.0 + ((c as i32) as f32 - 48.0);
                        }
                    },
                    _ => {
                        let digit = (c as i32) as f32 - 48.0;
                        tokens.push(Token::Number(digit));
                    }
                },
                '.' => {
                    is_calculating_decimals = true;
                    decimal_place = 0.1;
                },
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push(c);
                    is_calculating_decimals = false;
                },
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = parens.pop() {
                        if p != '(' {
                            return Err(Error::MismatchedParens);
                        }
                    } else {
                        return Err(Error::MismatchedParens);
                    }
                    is_calculating_decimals = false;
                },
                '+' => {
                    tokens.push(Token::Operator(Operator::Add));
                    is_calculating_decimals = false;
                },
                '-' => {
                    tokens.push(Token::Operator(Operator::Sub));
                    is_calculating_decimals = false;
                },
                '*' => {
                    tokens.push(Token::Operator(Operator::Mul));
                    is_calculating_decimals = false;
                },
                '/' => {
                    tokens.push(Token::Operator(Operator::Div));
                    is_calculating_decimals = false;
                },
                ' ' => is_calculating_decimals = false,
                '\n' => is_calculating_decimals = false,
                _ => return Err(Error::BadToken(c))
            }
        }

        if parens.len() > 0 {
            return Err(Error::MismatchedParens);
        }

        Ok(tokens)
    }

    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => {
                    queue.push(token);
                },
                Token::Operator(_) => {
                    while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('('){
                        if let Some(Token::Operator(last_stack_operator)) = stack.last().cloned() {
                            if let Token::Operator(current_operator) = token {
                                if last_stack_operator.value() >= current_operator.value() {
                                    queue.push(stack.pop().unwrap());
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    stack.push(token);
                },
                Token::Bracket('(') =>{
                    stack.push(token);
                },
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('(') {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.pop();
                },
                _ => {}
            }
        }

        while stack.len() > 0 {
            queue.push(stack.pop().unwrap());
        }

        queue
    }

    pub fn evaluate(mut tokens: Vec<Token>) -> Option<f32> {
        tokens.reverse();

        let mut stack: Vec<f32> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => stack.push(num as f32),
                Token::Operator(Operator::Add) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left + right);
                },
                Token::Operator(Operator::Sub) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left - right);
                },
                Token::Operator(Operator::Mul) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left * right);
                },
                Token::Operator(Operator::Div) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left / right);
                },
                _ => {}
            }
        }

        if stack.len() > 1 {
            None
        } else {
            stack.pop()
        }
    }

    pub fn resolve(expression: String) -> Option<String> {
        let tokens = Calculator::parse(expression.to_owned());
        let expr = Calculator::expression(tokens.unwrap());
        let result = Calculator::evaluate(expr)?;
    
        let result_string = if result.fract() == 0.0 {
            format!("{:.0}", result)
        } else {
            format!("{:?}", result)
        };
    
        Some(result_string)
    }

    pub fn count_char(char: char, expr: String) -> u32 {
        let mut char_count: u32 = 0;

        for c in expr.chars() {
            if c == char {
                char_count += 1;
            }
        }

        return char_count
    }

}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(char),
    Clear,
    Resolve,
}

impl Application for Calculator {

    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Calculator { expression: "0".to_owned(), result: None },
            Command::none()
        )
    }

    fn title(&self) -> String { "Rust Calculator".into() }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Edit(element) => {
                let mut owned_expression = self.expression.to_owned();

                if owned_expression.eq("0") && element != '.' {
                    owned_expression.pop();
                }

                let last_char_in_expression = owned_expression.chars().last().unwrap_or('?');

                if element == '.' && !last_char_in_expression.is_numeric() {
                    return Command::none()
                }

                if (element == '+' || element == '-' ||  element == '/' || element == '*') && (!last_char_in_expression.is_numeric() && last_char_in_expression != ')') {
                    return Command::none()
                }

                if element == ')' && !last_char_in_expression.is_numeric() {
                    return Command::none()
                }

                if element == '(' && (last_char_in_expression.is_numeric() || last_char_in_expression == '.' || last_char_in_expression == ')') {
                    return Command::none()
                }

                owned_expression.push(element);

                self.expression = owned_expression;
            },
            Message::Clear => {
                self.expression = String::new()
            },
            Message::Resolve => {
                self.result = Calculator::resolve(self.expression.to_owned());
                self.expression = self.result.clone().unwrap_or("".to_owned());
            },
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        container(
            column![
                vertical_space(),
                container(
                    text(self.expression.clone())
                        .size(40),
                )
                .padding(4)
                .width(screen_settings::get_app_width() - 8.0)
                .height(60)
                .center_y()
                .align_x(alignment::Horizontal::Right),
                // First Row
                row![
                    calculator_button::misc_action("C", false, Message::Clear),
                    calculator_button::misc_action("(", false, Message::Edit('(')),
                    calculator_button::misc_action(")", false, Message::Edit(')')),
                    calculator_button::operator_action("/", false),
                ]
                .spacing(ROW_SPACE),
                row![
                    calculator_button::number_action("7", false),
                    calculator_button::number_action("8", false),
                    calculator_button::number_action("9", false),
                    calculator_button::operator_action("*", false),
                ]
                .spacing(ROW_SPACE),
                row![
                    calculator_button::number_action("4", false),
                    calculator_button::number_action("5", false),
                    calculator_button::number_action("6", false),
                    calculator_button::operator_action("-", false),
                ]
                .spacing(ROW_SPACE),
                row![
                    calculator_button::number_action("1", false),
                    calculator_button::number_action("2", false),
                    calculator_button::number_action("3", false),
                    calculator_button::operator_action("+", false),
                ]
                .spacing(ROW_SPACE),
                row![
                    calculator_button::number_action("0", true),
                    calculator_button::number_action(".", false),
                    calculator_button::resolve_action("=", false),
                ]
                .spacing(ROW_SPACE)
            ]
            .spacing(COLUMN_SPACE)
        )
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

}