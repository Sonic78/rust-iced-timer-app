mod timer_cfg;
mod timer_ui;

use iced::{Application, Settings};
use timer_ui::TimerApp;

fn main() -> iced::Result {
    let settings = Settings {
        window: iced::window::Settings {
            size: (400, 300),
            min_size: Some((300, 280)),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    };
    TimerApp::run(settings)
}
