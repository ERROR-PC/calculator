use num_complex::Complex64;

use iced::{Sandbox, Theme, Length};
use iced::alignment::{Alignment, Horizontal, Vertical};

use iced::widget::{
    Column, Row, Container, Button, Text
};

use crate::button_enums::{MathFn, Operator};

#[derive(Debug, Clone, Copy)]
pub enum Pressed {
    Num(u8),
    Op(Operator),
    Const(Complex64),
    Func(MathFn),
}

#[derive(Debug, Clone, Default)]
pub struct Calculator {
    text: String,
}

macro_rules! num_btn {
    ($num: literal) => {
        iced::widget::Button::new(stringify!($num))
            .on_press(Pressed::Num($num))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
    };
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
            _ => {}
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        const PAD: u16 = 10;
        let display = Text::new(&self.text)
            .height(Length::Fill)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Left)
            .vertical_alignment(Vertical::Top);

        let button_plus = Button::new("+")
            .on_press(Pressed::Op(Operator::Plus))
            .width(Length::Fill)
            .height(Length::Fill);
        let button_minus = Button::new("-")
            .on_press(Pressed::Op(Operator::Minus))
            .width(Length::Fill)
            .height(Length::Fill);
        let button_mul = Button::new("×")
            .on_press(Pressed::Op(Operator::Mul))
            .width(Length::Fill)
            .height(Length::Fill);
        let button_divide = Button::new("÷")
            .on_press(Pressed::Op(Operator::Divide))
            .width(Length::Fill)
            .height(Length::Fill);

        let button_eq = Button::new("=")
            .on_press(Pressed::Op(Operator::Equal))
            .width(Length::Fill)
            .height(Length::Fill);

        let row1 = Row::new()
            .push(num_btn!(1))
            .spacing(PAD)
            .push(num_btn!(2))
            .spacing(PAD)
            .push(num_btn!(3))
            .spacing(PAD)
            .push(button_plus)
            .width(Length::Fill)
            .height(Length::Fill);

        let row2 = Row::new()
            .push(num_btn!(4))
            .spacing(PAD)
            .push(num_btn!(5))
            .spacing(PAD)
            .push(num_btn!(6))
            .spacing(PAD)
            .push(button_minus)
            .width(Length::Fill)
            .height(Length::Fill);

        let row3 = Row::new()
            .push(num_btn!(7))
            .spacing(PAD)
            .push(num_btn!(8))
            .spacing(PAD)
            .push(num_btn!(9))
            .spacing(PAD)
            .push(button_mul)
            .width(Length::Fill)
            .height(Length::Fill);

        let row4 = Row::new()
            .push(num_btn!(0).width(Length::FillPortion(2)))
            .spacing(PAD)
            .push(button_eq)
            .spacing(PAD)
            .push(button_divide)
            .width(Length::Fill)
            .height(Length::Fill);

        let non_disp_col = Column::new()
            .push(row1)
            .spacing(PAD)
            .push(row2)
            .spacing(PAD)
            .push(row3)
            .spacing(PAD)
            .push(row4)
            .width(Length::Fill)
            .height(Length::FillPortion(2));
        let all_column = Column::new()
            .push(display)
            .spacing(PAD)
            .push(non_disp_col)
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(all_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
