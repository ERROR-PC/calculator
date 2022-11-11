use iced::{
    Sandbox, Alignment, Theme
};

use iced::widget::{
    button, column, text
};

#[derive(Debug, Clone, Copy)]
pub enum Pressed {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Calculator {
    value: f64,
}

impl Sandbox for Calculator {
    type Message = Pressed;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("The le epic calculator")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Pressed::Plus => self.value += 1.0,
            Pressed::Minus => self.value -= 1.0,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        column![
            button("+").on_press(Pressed::Plus),
            text(self.value),
            button("-").on_press(Pressed::Minus),
        ]
            .padding(20)
            .align_items(Alignment::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
