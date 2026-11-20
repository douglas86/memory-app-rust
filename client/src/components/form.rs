//! # Form Component
//!
//! Provides the UI layout and handles all form data

use iced::Element;

#[derive(Default)]
pub struct Form {
    username: String,
}

#[derive(Debug, Clone)]
pub(crate) enum MsgForm {}

impl Form {
    pub fn form_update(&mut self, message: MsgForm) {}

    pub fn form_view(&self) -> Element<'_, MsgForm> {}
}
