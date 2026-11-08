use iced::widget::{Column, button, column, text};

#[derive(Default)]
pub struct App {
    value: u8,
}
#[derive(Debug, Clone, Copy)]
pub enum Msg {
    Increment,
    Decrement,
}

impl App {
    pub fn app_update(&mut self, message: Msg) {
        match message {
            Msg::Increment => {
                self.value += 1;
            }
            Msg::Decrement => {
                self.value -= 1;
            }
        }
    }

    pub fn app_view(&self) -> Column<'_, Msg> {
        let increment = button("+").on_press(Msg::Increment);
        let decrement = button("-").on_press_maybe((self.value > 0).then_some(Msg::Decrement));

        let counter = text(self.value);

        let interface = column![increment, counter, decrement];

        interface
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
