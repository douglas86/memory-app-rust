//! # Form Component
//!
//! Provides the UI layout and handles all form data

use iced::Element;
use iced::widget::text::Alignment;
use iced::widget::{Column, container, text_input};
use std::path::PathBuf;

#[derive(Default)]
pub struct Form {
    title: String,
    image: Option<PathBuf>,
    description: String,
    tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) enum MsgForm {
    TitleChanged(String),
}

impl Form {
    pub fn form_update(&mut self, message: MsgForm) {
        match message {
            MsgForm::TitleChanged(title) => {
                self.title = title;
                println!("Title: {}", self.title);
            }
        }
    }

    pub fn form_view(&self) -> Element<'_, MsgForm> {
        let form = container(Column::new().spacing(20).align_x(Alignment::Center).push(
            text_input("Type something here...", &self.title).on_input(MsgForm::TitleChanged),
        ));

        container(form).into()
    }
}
