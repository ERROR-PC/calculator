use num_complex::Complex64;

use iced::{Sandbox, Theme, Length};
use iced::alignment::{Horizontal, Vertical};

use iced::widget::{
    column, row, container, text
};

use crate::button_enums::{MathFn, Operator};
use crate::button_funcs::{num_container, basic_ops};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
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

impl Calculator {
    #[inline]
    pub fn is_num_start(&self) -> bool {
        !self.text.chars().last().unwrap_or('+').is_numeric()
    }
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
            Pressed::Num(num) => {
                if self.is_num_start() && num == 0 {
                    return;
                }
                self.text.push((num + 48) as char)
            },
            Pressed::Op(op) => {
                if self.is_num_start() {
                    return;
                }
                self.text.push(op.into());
            },
            _ => {

            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        const PAD: u16 = 10;
        let display = text(&self.text)
            .height(Length::Fill)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Left)
            .vertical_alignment(Vertical::Top);

        let nums = num_container();
        let ops = basic_ops();

        let non_disp_row = row![nums, ops]
            .spacing(PAD)
            .width(Length::Fill)
            .height(Length::FillPortion(2));
        let all_column = column![display, non_disp_row]
            .spacing(PAD)
            .width(Length::Fill)
            .height(Length::Fill);

        container(all_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(PAD)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
