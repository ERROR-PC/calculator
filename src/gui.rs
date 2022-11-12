use num_complex::Complex64;

use iced::{
    Sandbox, Alignment, Theme
};

use iced::widget::{
    button, column, text, row, container
};

use crate::button_enums::{Operator, MathFn};

#[derive(Debug, Clone, Copy)]
pub enum Pressed {
    Num(u8),
    Op(Operator),
    Const(Complex64),
    Func(MathFn),
}

#[derive(Debug, Clone, Default)]
pub struct Calculator {
    text: String
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
            // 48 is the begining of digits in ascii
            Pressed::Num(num) => self.text.push((num + 48) as char),
            _ => {},
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        const PAD: u16 = 5;
        const SPACE: u16 = 10;
        column![
            container(text(&self.text)).height(30.into()),
            row![
                button("1").on_press(Pressed::Num(1)),
                button("2").on_press(Pressed::Num(2)),
                button("3").on_press(Pressed::Num(3)),
            ].padding(PAD).spacing(SPACE),
            row![
                button("4").on_press(Pressed::Num(4)),
                button("5").on_press(Pressed::Num(5)),
                button("6").on_press(Pressed::Num(6)),
            ].padding(PAD).spacing(SPACE),
            row![
                button("7").on_press(Pressed::Num(7)),
                button("8").on_press(Pressed::Num(8)),
                button("9").on_press(Pressed::Num(9)),
            ].padding(PAD).spacing(SPACE),
            row![
                button("0").on_press(Pressed::Num(0)),
                button("=").on_press(Pressed::Op(Operator::Equal)),
            ].padding(PAD).spacing(SPACE),
        ]
            .spacing(PAD)
            .padding(PAD)
            .align_items(Alignment::Center)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
