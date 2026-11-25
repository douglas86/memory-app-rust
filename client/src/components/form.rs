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
    tags: String,
}

#[derive(Debug, Clone)]
pub(crate) enum MsgForm {
    TitleChanged(String),
    DescriptionChanged(String),
    TagChanged(String),
}

impl Form {
    pub fn form_update(&mut self, message: MsgForm) {
        match message {
            MsgForm::TitleChanged(title) => {
                self.title = title;
                println!("Title: {}", self.title);
            }
            MsgForm::DescriptionChanged(description) => {
                self.description = description;
                println!("Description: {}", self.description);
            }
            MsgForm::TagChanged(tag) => {
                self.tags = tag;
                println!("Tag: {}", self.tags);
            }
        }
    }

    pub fn form_view(&self) -> Element<'_, MsgForm> {
        let form = container(
            Column::new()
                .spacing(20)
                .align_x(Alignment::Center)
                .push(text_input("Title...", &self.title).on_input(MsgForm::TitleChanged))
                .push(
                    text_input("Description...", &self.description)
                        .on_input(MsgForm::DescriptionChanged),
                )
                .push(
                    text_input("Tags (comma-separated)...", &self.tags)
                        .on_input(MsgForm::TagChanged),
                ),
        );

        container(form).into()
    }
}
