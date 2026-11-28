//! # Modal Component
//!
//! Provides the UI dialog overlay and state handler for user model

use crate::components::form::{Form, MsgForm};
use iced::widget::{Column, Row, button, container, text};
use iced::{Alignment, Color, Element, Length, Task};

#[derive(Default)]
pub struct Modal {
    pub show_modal: bool,
    form: Form,
}

#[derive(Debug, Clone)]
pub(crate) enum MsgModal {
    OpenModal,  // Opens the model when called
    CloseModal, // Closed the model when called
    Submit,     // Submit the data from model
    Form(MsgForm),
}

impl Modal {
    pub fn modal_update(&mut self, message: MsgModal) -> Task<MsgModal> {
        match message {
            MsgModal::OpenModal => {
                self.show_modal = true;

                Task::none()
            }
            MsgModal::CloseModal => {
                self.show_modal = false;
                self.form = Form::default();

                Task::none()
            }
            MsgModal::Submit => {
                self.show_modal = false;

                let f = self.form.return_form_data();
                println!("{:?}", f);

                self.form = Form::default();

                Task::none()
            }
            MsgModal::Form(form) => self.form.form_update(form).map(MsgModal::Form),
        }
    }

    pub fn modal_view(&self) -> Element<'_, MsgModal> {
        let buttons = Row::new()
            .push(
                button("Submit")
                    .on_press(MsgModal::Submit)
                    .style(|_theme, _status| button::Style {
                        background: Some(iced::Background::Color(Color::from_rgb8(179, 196, 26))),
                        text_color: Color::WHITE,
                        border: iced::Border {
                            radius: 6.0.into(),
                            ..iced::Border::default()
                        },
                        ..button::Style::default()
                    }),
            )
            .push(
                button("Close")
                    .on_press(MsgModal::CloseModal)
                    .style(|_theme, _status| button::Style {
                        background: Some(iced::Background::Color(Color::from_rgb8(34, 13, 189))),
                        text_color: Color::WHITE,
                        border: iced::Border {
                            radius: 6.0.into(),
                            ..iced::Border::default()
                        },
                        ..button::Style::default()
                    }),
            )
            .spacing(20);

        let model_box = container(
            Column::new()
                .spacing(20)
                .align_x(Alignment::Center)
                .push(text!("Create New Modal").size(24))
                .push(self.form.form_view().map(MsgModal::Form))
                .push(buttons),
        )
        .width(Length::Fill)
        .max_width(500.0)
        .padding(20)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb8(52, 235, 149))),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_default_is_closed() {
        let modal = Modal::default();
        assert_eq!(modal.show_modal, false);
    }

    #[test]
    fn test_modal_update_state() {
        let mut modal = Modal::default();

        let _ = modal.modal_update(MsgModal::CloseModal);
        assert_eq!(modal.show_modal, false);
    }

    #[test]
    fn test_modal_update_state_is_open() {
        let mut modal = Modal::default();

        let _ = modal.modal_update(MsgModal::OpenModal);
        assert_eq!(modal.show_modal, true);
    }

    #[test]
    fn test_model_view_construction() {
        let model_closed = Modal::default();
        let model_open = Modal {
            show_modal: true,
            form: Default::default(),
        };

        let _closed_element = model_closed.modal_view();
        let _open_element = model_open.modal_view();
    }

    #[test]
    fn test_modal_open_and_close_lifecycle() {
        let mut modal = Modal::default();

        // Initial state: Modal is closed
        assert_eq!(modal.show_modal, false);

        // User clicks the "Open" button (triggers MsgModal::OpenModal)
        let _ = modal.modal_update(MsgModal::OpenModal);
        assert_eq!(modal.show_modal, true);

        // User clicks the "Close" button inside the modal (triggers MsgModal::CloseModal)
        let _ = modal.modal_update(MsgModal::CloseModal);
        assert_eq!(modal.show_modal, false);
    }
}
