use calculator::Calculator;

use iced::window::settings::PlatformSpecific;
use iced::{Application, Settings, Size};
use iced::window::{self, Level, Position};

pub mod calculator;
use calculator::screen_settings::{get_app_width, get_app_height};

pub fn main() -> iced::Result {
    let settings: Settings<()> = Settings {
        window: window::Settings { 
            size: Size { width: get_app_width(), height: get_app_height() }, 
            position: Position::Centered, 
            min_size: Some(Size { width: get_app_width(), height: get_app_height() }), 
            max_size: Some(Size { width: get_app_width(), height: get_app_height() }), 
            visible: true, 
            resizable: false, 
            decorations: true, 
            transparent: false, 
            level: Level::Normal, 
            icon: None, 
            platform_specific: PlatformSpecific { 
                title_hidden: true, 
                titlebar_transparent: true, 
                fullsize_content_view: false 
            }, 
            exit_on_close_request: true,
        },
        ..Default::default()
    };

    Calculator::run(settings)
}