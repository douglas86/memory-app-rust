//! # Form Component
//!
//! Provides the UI layout and handles all form data

use iced::Element;
use std::path::PathBuf;

#[derive(Default)]
pub struct Form {
    title: String,
    image: Option<PathBuf>,
    description: String,
    tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) enum MsgForm {}

impl Form {
    pub fn form_update(&mut self, message: MsgForm) {}

    pub fn form_view(&self) -> Element<'_, MsgForm> {}
}
