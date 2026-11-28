//! # Form Component
//!
//! Provides the UI layout and handles all form data

use iced::widget::text::Alignment;
use iced::widget::{Column, button, container, text_input};
use iced::{Color, Element, Task};
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
    ImageChanged(Option<PathBuf>),
    OpenImagePicker,
}

impl Form {
    pub fn form_update(&mut self, message: MsgForm) -> Task<MsgForm> {
        match message {
            MsgForm::TitleChanged(title) => {
                self.title = title;
                println!("Title: {}", self.title);

                Task::none()
            }
            MsgForm::DescriptionChanged(description) => {
                self.description = description;
                println!("Description: {}", self.description);

                Task::none()
            }
            MsgForm::TagChanged(tag) => {
                self.tags = tag;
                println!("Tag: {}", self.tags);

                Task::none()
            }
            MsgForm::ImageChanged(path) => {
                self.image = path;

                if let Some(path) = &self.image {
                    println!("Image path: {:?}", path);
                }

                Task::none()
            }
            MsgForm::OpenImagePicker => Task::perform(Form::pick_image(), MsgForm::ImageChanged),
        }
    }

    async fn pick_image() -> Option<PathBuf> {
        let file = rfd::AsyncFileDialog::new()
            .add_filter("*.png", &["png", "png", "jpg", "jpeg"])
            .set_directory("/")
            .pick_file()
            .await;

        file.map(|f| f.path().to_path_buf())
    }

    // TODO: The images picker seems to be working in for form.rs file without any errors
    // TODO: The next step is to wire it up to the modal section

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
                )
                .push(
                    button("Select Image")
                        .on_press(MsgForm::OpenImagePicker)
                        .style(|_theme, _status| button::Style {
                            background: Some(iced::Background::Color(Color::from_rgb8(19, 82, 36))),
                            text_color: Color::WHITE,
                            border: iced::Border {
                                radius: 6.0.into(),
                                ..iced::Border::default()
                            },
                            ..button::Style::default()
                        }),
                ),
        );

        container(form).into()
    }
}
