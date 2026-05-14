mod timer_cfg;
mod timer_ui;

use iced::{Application, Settings};
use timer_ui::TimerApp;

fn main() -> iced::Result {
    let settings = Settings {
        window: iced::window::Settings {
            size: (320, 280),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    };
    TimerApp::run(settings)
}
