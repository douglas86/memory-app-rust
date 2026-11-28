//! # Form Component
//!
//! Provides the UI layout and handles all form data

use iced::widget::{Column, Row, button, container, text, text_input};
use iced::{Alignment, Color, Element, Task};
use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
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

    pub fn return_form_data(&self) -> &Self {
        self
    }

    pub fn form_view(&self) -> Element<'_, MsgForm> {
        let image_picker = Row::new()
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
            )
            .push(text(
                self.image
                    .as_deref()
                    .and_then(|f| {
                        let truncate_at_character = 20;
                        let name = f.file_name()?.to_str()?;

                        if name.chars().count() > truncate_at_character {
                            let truncate =
                                name.chars().take(truncate_at_character).collect::<String>();
                            Some(format!("{}...", truncate))
                        } else {
                            Some(name.to_string())
                        }
                    })
                    .unwrap_or("No image selected".to_string()),
            ))
            .align_y(Alignment::Center)
            .spacing(8);

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
                .push(image_picker),
        );

        container(form).into()
    }
}
