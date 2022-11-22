mod constants;
mod errors;
mod gui;

pub use constants::*;
use gui::Calculator;
use iced::{window, Application, Settings};

fn main() -> iced::Result {
    Calculator::run(Settings {
        // change the default size to be small
        window: window::Settings { size: (240, 320), ..Default::default() },
        text_multithreading: true,
        antialiasing: false,
        ..Default::default()
    })
}
