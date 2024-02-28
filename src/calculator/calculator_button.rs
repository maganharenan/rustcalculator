use iced::{Background, Border, Element, Theme};
use iced::widget::{button, container, text};
use iced::color;

use crate::calculator::screen_settings::{BUTTON_WIDTH, BUTTON_HEIGHT, ROW_SPACE};

use crate::calculator;

#[derive(Default)]
pub enum ButtonStyle {
    /// The primary style.
    #[default]
    Number,
    Operator,
    Equal,
    Misc,
}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: color!(0xffffff).into(),
            background: get_background_color(self),
            border: Border::with_radius(0),
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: color!(0xffffff).into(),
            background: get_hovered_background_color(self),
            border: Border::with_radius(0),
            ..Default::default()
        } 
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: color!(0x666666).into(),
            background: get_pressed_background_color(self),
            border: Border::with_radius(0),
            ..Default::default()
        }  
    }

}

fn get_background_color(_style: &ButtonStyle) ->Option< Background> {
    match _style {
        ButtonStyle::Number => Some(Background::from(color!(0x666666))),
        ButtonStyle::Operator => Some(Background::from(color!(0x103f61))),
        ButtonStyle::Equal => Some(Background::from(color!(0x103f61))),
        ButtonStyle::Misc => Some(Background::from(color!(0x3a3a3a))),
    }
}

fn get_hovered_background_color(_style: &ButtonStyle) ->Option< Background> {
    match _style {
        ButtonStyle::Number => Some(Background::from(color!(0x9c9c9c))),
        ButtonStyle::Operator => Some(Background::from(color!(0x165a8a))),
        ButtonStyle::Equal => Some(Background::from(color!(0x165a8a))),
        ButtonStyle::Misc => Some(Background::from(color!(0x545454))),
    }
}

fn get_pressed_background_color(_style: &ButtonStyle) ->Option< Background> {
    match _style {
        ButtonStyle::Number => Some(Background::from(color!(0x474747))),
        ButtonStyle::Operator => Some(Background::from(color!(0x09263b))),
        ButtonStyle::Equal => Some(Background::from(color!(0x09263b))),
        ButtonStyle::Misc => Some(Background::from(color!(0x171717))),
    }
}

pub fn number_action<'a>(label: &'a str, double_size: bool) -> Element<calculator::Message, Theme> {
    let size_multiplier: f32 = if double_size { 2.0 } else { 1.0 };
    let aditional_space: f32 = if double_size { ROW_SPACE } else { 0.0 };

    button(
        container(
            text(label)
            .size(22)
        )
        .width(BUTTON_WIDTH * size_multiplier + aditional_space)
        .height(BUTTON_HEIGHT)
        .center_x()
        .center_y()
    )
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Number)))
        .width(BUTTON_WIDTH * size_multiplier + aditional_space)
        .height(BUTTON_HEIGHT)
        .on_press(calculator::Message::Edit(label.chars().next().unwrap()))
        .into() 
}

pub fn operator_action<'a>(label: &'a str, double_size: bool) -> Element<calculator::Message, Theme> {
    let size_multiplier: f32 = if double_size { 2.0 } else { 1.0 };
    let aditional_space: f32 = if double_size { ROW_SPACE } else { 0.0 };

    button(
        container(
            text(label)
            .size(22)
        )
        .width(BUTTON_WIDTH * size_multiplier + aditional_space)
        .height(BUTTON_HEIGHT)
        .center_x()
        .center_y()
    )
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Operator)))
        .width(BUTTON_WIDTH * size_multiplier + aditional_space)
        .height(BUTTON_HEIGHT)
        .on_press(calculator::Message::Edit(label.chars().next().unwrap()))
        .into() 
}

pub fn resolve_action<'a>(label: &'a str, double_size: bool) -> Element<calculator::Message, Theme> {
    let size_multiplier: f32 = if double_size { 2.0 } else { 1.0 };
    let aditional_space: f32 = if double_size { ROW_SPACE } else { 0.0 };

    button(
        container(
            text(label)
            .size(22)
        )
        .width(BUTTON_WIDTH * size_multiplier + aditional_space)
        .height(BUTTON_HEIGHT)
        .center_x()
        .center_y()
    )
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Equal)))
        .width(BUTTON_WIDTH * size_multiplier + aditional_space)
        .height(BUTTON_HEIGHT)
        .on_press(calculator::Message::Resolve)
        .into() 
}

pub fn misc_action<'a>(label: &'a str, double_size: bool, message: calculator::Message) -> Element<calculator::Message, Theme> {
    let size_multiplier: f32 = if double_size { 2.0 } else { 1.0 };
    let aditional_space: f32 = if double_size { ROW_SPACE } else { 0.0 };

    button(
        container(
            text(label)
            .size(22)
        )
        .width(BUTTON_WIDTH * size_multiplier + aditional_space)
        .height(BUTTON_HEIGHT)
        .center_x()
        .center_y()
    )
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Misc)))
        .width(BUTTON_WIDTH * size_multiplier + aditional_space)
        .height(BUTTON_HEIGHT)
        .on_press(message)
        .into() 
}