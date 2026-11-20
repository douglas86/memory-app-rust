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

        modal.modal_update(MsgModal::CloseModal);
        assert_eq!(modal.show_modal, false);
    }

    #[test]
    fn test_modal_update_state_is_open() {
        let mut modal = Modal::default();

        modal.modal_update(MsgModal::OpenModal);
        assert_eq!(modal.show_modal, true);
    }

    #[test]
    fn test_model_view_construction() {
        let model_closed = Modal::default();
        let model_open = Modal { show_modal: true };

        let _closed_element = model_closed.modal_view();
        let _open_element = model_open.modal_view();
    }

    #[test]
    fn test_modal_open_and_close_lifecycle() {
        let mut modal = Modal::default();

        // Initial state: Modal is closed
        assert_eq!(modal.show_modal, false);

        // User clicks the "Open" button (triggers MsgModal::OpenModal)
        modal.modal_update(MsgModal::OpenModal);
        assert_eq!(modal.show_modal, true);

        // User clicks the "Close" button inside the modal (triggers MsgModal::CloseModal)
        modal.modal_update(MsgModal::CloseModal);
        assert_eq!(modal.show_modal, false);
    }
}
