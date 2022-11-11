mod gui;

use iced::{Sandbox, Settings, window};
use gui::Calculator;

fn main() -> iced::Result {
    Calculator::run(
        Settings {
            window: window::Settings { size: (360, 640), ..Default::default() },
            text_multithreading: true,
            antialiasing: false,
            ..Default::default()
        }
    )
}
