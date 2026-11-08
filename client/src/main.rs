mod app;

use app::App;

fn main() -> iced::Result {
    iced::run(App::app_update, App::app_view)
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
