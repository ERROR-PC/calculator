use iced::widget::{
    Container, container, row, column, Column, button
};
use iced::Length;

use crate::gui_enums::Operator;
use crate::gui::Pressed;

const PAD: u16 = 10;
pub fn num_container<'a>() -> Container<'a, Pressed> {
    macro_rules! num_btn {
        ($num: literal) => {
            iced::widget::button(stringify!($num))
                .on_press(crate::gui::Pressed::Num($num))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
        };
    }

    let row1 = row![num_btn!(1), num_btn!(2), num_btn!(3)]
        .spacing(PAD)
        .width(Length::Fill)
        .height(Length::Fill);

    let row2 = row![num_btn!(4), num_btn!(5), num_btn!(6)]
        .spacing(PAD)
        .width(Length::Fill)
        .height(Length::Fill);

    let row3 = row![num_btn!(7), num_btn!(8), num_btn!(9)]
        .spacing(PAD)
        .width(Length::Fill)
        .height(Length::Fill);

    let btn_eq = button("=")
        .on_press(Pressed::Op(Operator::Equal))
        .width(Length::FillPortion(1))
        .height(Length::Fill);
    let row4 = row![
        num_btn!(0).width(Length::FillPortion(2)),
        btn_eq,
    ]
        .spacing(PAD)
        .width(Length::Fill)
        .height(Length::Fill);

    let all_cols = column![row1, row2, row3, row4]
        .spacing(PAD)
        .width(Length::Fill)
        .height(Length::Fill);

    container(all_cols)
        .center_x()
        .center_y()
        .height(Length::Fill)
        .width(Length::FillPortion(3)) // 3 columns
}

pub fn basic_ops<'a>() -> Column<'a, Pressed> {
    let plus = button("+")
        .on_press(Pressed::Op(Operator::Plus))
        .height(Length::Fill)
        .width(Length::Fill);

    let minus = button("-")
        .on_press(Pressed::Op(Operator::Minus))
        .height(Length::Fill)
        .width(Length::Fill);

    let mul = button("×")
        .on_press(Pressed::Op(Operator::Mul))
        .height(Length::Fill)
        .width(Length::Fill);

    let divide = button("÷")
        .on_press(Pressed::Op(Operator::Divide))
        .height(Length::Fill)
        .width(Length::Fill);

    column![plus, minus, mul, divide]
        .spacing(PAD)
        .height(Length::Fill)
        .width(Length::Fill)
}
