mod gui;
mod button_enums;
mod button_funcs;

use iced::{Sandbox, Settings, window};
use gui::Calculator;

fn main() -> iced::Result {
    Calculator::run(
        Settings {
            window: window::Settings { size: (240, 320), ..Default::default() },
            text_multithreading: true,
            antialiasing: false,
            ..Default::default()
        }
    )
}
