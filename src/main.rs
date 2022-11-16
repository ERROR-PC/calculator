mod gui;
mod gui_enums;
mod gui_funcs;
mod errors;

use iced::{Application, Settings, window};
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
