use iced::{Sandbox, Button, Text};

#[derive(Debug, Clone, Copy, Default)]
pub struct Calculator;

impl Sandbox for Calculator {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("A test application!")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Text::new("Hello world!").into()
    }

    fn update(&mut self, _message: Self::Message) {
        
    }
}