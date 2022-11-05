use iced::{Sandbox, button, Alignment, Color, Column};
use iced::widget::{Button, Text};

#[derive(Debug, Clone, Copy)]
pub enum ButtonPressed {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Calculator {
    value: f64,

    plus_button: button::State,
    minus_button: button::State,
}

impl Sandbox for Calculator {
    type Message = ButtonPressed;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("The le epic error calculator")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.plus_button, Text::new("+"))
                .on_press(ButtonPressed::Plus)
            )
            .push(
                Text::new(self.value.to_string())
                .color(Color::WHITE)
            )
            .push(
                Button::new(&mut self.minus_button, Text::new("-"))
                    .on_press(ButtonPressed::Minus)
            )
            .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            ButtonPressed::Plus => {
                self.value += 1.0;
            },
            ButtonPressed::Minus => {
                self.value -= 1.0;
            },
        }
    }

    fn background_color(&self) -> Color {
        Color::from_rgb8(30, 30, 30)
    }
}
