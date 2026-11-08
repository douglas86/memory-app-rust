mod app;

use app::App;

fn main() -> iced::Result {
    iced::run(App::app_update, App::app_view)
}
