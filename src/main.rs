mod gui;

use iced::{Sandbox, Settings};
use gui::Calculator;

fn main() -> iced::Result {
    Calculator::run(Settings::default())
}
