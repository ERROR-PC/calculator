mod enums;
mod funcs;

use num_complex::Complex64;

use iced::{Theme, Length, Application, Command};
use iced::alignment::{Horizontal, Vertical};

use iced::widget::{
    column, row, container, text
};

use crate::gui::enums::{MathFn, Operator};
use crate::gui::funcs::{num_container, basic_ops};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Pressed {
    Num(u8),
    Op(Operator),
    Const(Complex64),
    Func(MathFn),
    Keyboard(iced::keyboard::Event),
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

impl Application for Calculator {
    type Executor = iced::executor::Default;
    type Message = Pressed;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("The le epic calculator")
    }

    fn update(&mut self, message: Self::Message) -> Command<Pressed> {
        const ASCII_OF_0: u8 = 48;
        match message {
            Pressed::Num(num) => {
                if self.is_num_start() && num == 0 {
                    return Command::none();
                }
                self.text.push((num + ASCII_OF_0) as char)
            },
            Pressed::Op(op) => {
                if self.is_num_start() {
                    return Command::none();
                }

                self.text.push(op.into());
            },
            Pressed::Keyboard(event) => {
                use iced::keyboard::Event;

                match event {
                    Event::CharacterReceived(ch) => {
                        if ch.is_numeric() {
                            self.update(Pressed::Num(ch as u8 - ASCII_OF_0));
                        }
                        else if let Ok(operator) = ch.try_into() {
                            self.update(Pressed::Op(operator));
                        }
                    },
                    Event::ModifiersChanged(_modifiers) => {}, // idk if I will use this
                    event => eprintln!("Error ignored: some weird keyboard event was sent\nLine {}, file {}\nEvent is\n{:?}", line!(), file!(), event),
                }
            },
            _ => {
                todo!()
            },
        }

        Command::none()
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

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        use iced::keyboard::Event::{CharacterReceived, ModifiersChanged};
        use iced::subscription::events_with;

        events_with(|event, _status|
            if let iced::Event::Keyboard(CharacterReceived(ch)) = event {
                Some(Pressed::Keyboard(CharacterReceived(ch)))
            }
            else if let iced::Event::Keyboard(ModifiersChanged(modifiers)) = event {
                Some(Pressed::Keyboard(ModifiersChanged(modifiers)))
            }
            else { None }
        )
    }
}
