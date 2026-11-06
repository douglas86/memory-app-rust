use iced::widget::{Column, button, column, text};

#[derive(Default)]
struct Counter {
    value: u8,
}

#[derive(Debug, Clone, Copy)]
enum Msg {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Msg) {
        match message {
            Msg::Increment => {
                self.value += 1;
            }
            Msg::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Column<'_, Msg> {


        let increment = button("+").on_press(Msg::Increment);
        let decrement = button("-").on_press_maybe((self.value > 0).then_some(Msg::Decrement));

        let counter = text(self.value);

        let interface = column![increment, counter, decrement];

        interface
    }
}

fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}

#[cfg(test)]
mod tests {
    use crate::{Counter, Msg};

    #[test]
    fn it_counts_properly() {
        let mut counter = Counter { value: 0 };

        counter.update(Msg::Increment);
        counter.update(Msg::Increment);
        counter.update(Msg::Decrement);

        assert_eq!(counter.value, 1);
    }
}
