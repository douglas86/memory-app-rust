use iced::widget::{Column, button, container, text};
use iced::{Alignment, Length};
use iced::{Color, Element};

#[derive(Default)]
pub struct App {
    show_modal: bool,
}
#[derive(Debug, Clone)]
pub enum Msg {
    OpenModal,
    CloseModal,
}

impl App {
    pub fn app_update(&mut self, message: Msg) {
        match message {
            Msg::OpenModal => {
                self.show_modal = true;
            }
            Msg::CloseModal => {
                self.show_modal = false;
            }
        }
    }

    pub fn app_view(&self) -> Element<'_, Msg> {
        //creates the model
        let modal = container(
            Column::new()
                .spacing(20)
                .align_x(Alignment::Center)
                .push(text!("Create New Model").size(24))
                .push(button("Close").on_press(Msg::CloseModal)),
        )
        .padding(20)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb8(35, 38, 45))),
            border: iced::Border {
                radius: 10.0.into(),
                width: 1.0,
                color: Color::from_rgb8(60, 65, 75),
            },
            ..container::Style::default()
        });

        // model that is needing to be displayed
        let main_content = container(modal)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgba(
                    0.0, 0.0, 0.0, 0.6,
                ))),
                ..container::Style::default()
            });

        // show model only if variable true
        if !self.show_modal {
            return main_content.into();
        }

        // TODO: Create the main content here
        // TODO: The above code is all todo with the modal
    }
}

#[cfg(test)]
mod tests {
    use crate::App;
    use crate::app::Msg;

    #[test]
    fn test_it_counts_properly() {
        let mut counter = App { value: 0 };

        counter.app_update(Msg::Increment);
        counter.app_update(Msg::Increment);
        counter.app_update(Msg::Decrement);

        assert_eq!(counter.value, 1);
    }
}
