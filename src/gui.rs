use iced::{
    Sandbox
};
use iced::widget::{
    button, column, text, Column
};

#[derive(Debug, Clone, Copy)]
pub enum ButtonPressed {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Calculator {
    value: f64,
}

impl Sandbox for Calculator {
    type Message = ButtonPressed;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("The le epic calculator")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            ButtonPressed::Plus => self.value += 1.0,
            ButtonPressed::Minus => self.value -= 1.0,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        column![
            button("+").on_press(ButtonPressed::Plus),
        ]
    }
}
