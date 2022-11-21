mod gui;
mod errors;
mod constants;

use iced::{Application, Settings, window};
use gui::Calculator;
pub use constants::*;

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
