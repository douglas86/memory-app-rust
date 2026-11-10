//! # Modal Component
//!
//! Provides the UI dialog overlay and state handler for user model

use iced::widget::{Column, button, container, text};
use iced::{Alignment, Color, Element, Length};

#[derive(Default)]
pub struct Modal {
    pub show_modal: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum MsgModal {
    OpenModal,  // Opens the model when called
    CloseModal, // Closed the model when called
}

impl Modal {
    pub fn modal_update(&mut self, message: MsgModal) {
        match message {
            MsgModal::OpenModal => {
                self.show_modal = true;
            }
            MsgModal::CloseModal => {
                self.show_modal = false;
            }
        }
    }

    pub fn modal_view(&self) -> Element<'_, MsgModal> {
        let model_box = container(
            Column::new()
                .spacing(20)
                .align_x(Alignment::Center)
                .push(text!("Create New Modal").size(24))
                .push(button("close").on_press(MsgModal::CloseModal)),
        )
        .padding(20)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb8(35, 38, 45))),
            border: iced::Border {
                radius: 10.0.into(),
                width: 1.0,
                color: Color::from_rgb8(60, 65, 72),
            },
            ..container::Style::default()
        });

        container(model_box)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgba(
                    0.0, 0.0, 0.0, 0.6,
                ))),
                ..container::Style::default()
            })
            .into()
    }
}
