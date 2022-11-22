mod enums;
mod funcs;
mod structs;

use std::str::FromStr;

use num_complex::Complex64;

use iced::{Theme, Length, Application, Command};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    column, row, container, text
};

use crate::gui::{
    funcs::{num_container, basic_ops},
    enums::{Pressed, Operator},
    structs::Token,
};
use crate::constants::*;

/// This is the entire calculator, implements Application
#[derive(Debug, Clone, Default)]
pub struct Calculator {
    tokens: Vec<Token>,
}

impl Calculator {
    /// Checks wether this is the beginning of a new number
    #[inline]
    pub fn is_num_start(&self) -> bool {
        !matches!(self.tokens.last(), Some(Token::Num(_)))
    }

    /// Evaluates the tokens into a complex number
    pub fn eval(&mut self) {
        
    }
}

/// The text display widget's text is implemented here
impl std::fmt::Display for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.tokens {
            write!(f, "{}", token)?;
        }
        Ok(())
    }
}

impl Application for Calculator {
    type Executor = iced::executor::Default;
    type Message = Pressed;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("The le epic calculator")
    }

    fn update(&mut self, message: Self::Message) -> Command<Pressed> {
        match message {
            Pressed::Num(num) => {
                if self.is_num_start() && num == 0 {
                    return Command::none();
                }
                else if !self.is_num_start() {
                    let mut temp = String::new();
                    let last_token = self.tokens.last_mut().expect("There *should* be a number before this one");
                    temp.push_str(&last_token.to_string());
                    temp.push((num + ASCII_OF_0) as char);
                    *last_token = Token::from_str(&temp)
                            .expect("Logical error: Num variant does not contain a num");
                }
                else {
                    self.tokens.push(Token::Num((num as f64).into()))
                }
        },
            Pressed::Op(op) => {
                if self.is_num_start() {
                    return Command::none();
                }

                if op == Operator::Equal {
                    self.eval();
                    return Command::none();
                }
                self.tokens.push(Token::Op(op));
            },
            Pressed::Const(_num) => {
                /* todo! */
            },
            Pressed::Keyboard(event) => {
                use iced::keyboard::Event;

                match event {
                    Event::CharacterReceived(ch) => {
                        if ch.is_numeric() {
                            self.update(Pressed::Num(ch as u8 - ASCII_OF_0));
                        }
                        else if ch == 'i' {
                            self.update(Pressed::Const(Complex64::i()));
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
        let display = text(&self.to_string())
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
        use iced::Event::Keyboard;
        use iced::subscription::events_with;

        events_with(|event, _status|
            if let Keyboard(CharacterReceived(ch)) = event {
                Some(Pressed::Keyboard(CharacterReceived(ch)))
            }
            else if let Keyboard(ModifiersChanged(modifiers)) = event {
                Some(Pressed::Keyboard(ModifiersChanged(modifiers)))
            }
            else { None }
        )
    }
}
