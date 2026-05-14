use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Application, Command, Element, Length, Settings, theme};
use serde::{Deserialize, Serialize};
use std::time::Instant;

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

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    timer: TimerConfig,
}

#[derive(Serialize, Deserialize, Debug)]
struct TimerConfig {
    red_text_limit_seconds: u64,
}

fn load_config() -> Config {
    let config_str = std::fs::read_to_string("config.toml")
        .unwrap_or_else(|_| String::from("[timer]\nred_text_limit_seconds = 120"));
    toml::from_str(&config_str).unwrap_or_else(|_| Config {
        timer: TimerConfig {
            red_text_limit_seconds: 120,
        },
    })
}

struct TimerApp {
    elapsed_time: std::time::Duration,
    is_running: bool,
    start_time: Option<Instant>,
    paused_time: std::time::Duration,
    limit_for_red_color_in_seconds: u64,
}

#[derive(Debug, Clone)]
enum Message {
    Start,
    Stop,
    Reset,
    Tick,
}

impl Application for TimerApp {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let config = load_config();
        (
            TimerApp {
                elapsed_time: std::time::Duration::from_secs(0),
                is_running: false,
                start_time: None,
                paused_time: std::time::Duration::from_secs(0),
                limit_for_red_color_in_seconds: config.timer.red_text_limit_seconds,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Timer Application")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Start => {
                if !self.is_running {
                    self.is_running = true;
                    self.start_time = Some(Instant::now());
                }
            }
            Message::Stop => {
                if self.is_running {
                    self.is_running = false;
                    // Update elapsed time to the final value
                    if let Some(start) = self.start_time {
                        self.paused_time += start.elapsed();
                        self.elapsed_time = self.paused_time;
                    }
                    self.start_time = None;
                }
            }
            Message::Reset => {
                self.is_running = false;
                self.elapsed_time = std::time::Duration::from_secs(0);
                self.paused_time = std::time::Duration::from_secs(0);
                self.start_time = None;
            }
            Message::Tick => {
                if self.is_running {
                    if let Some(start) = self.start_time {
                        self.elapsed_time = self.paused_time + start.elapsed();
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let minutes = self.elapsed_time.as_secs() / 60;
        let seconds = self.elapsed_time.as_secs() % 60;
        let display = format!("{:02}:{:02}", minutes, seconds);

        let time_text = if self.elapsed_time.as_secs() >= self.limit_for_red_color_in_seconds {
            text(display).size(80).style(iced::Color::from_rgb(1.0, 0.0, 0.0))
        } else {
            text(display).size(80).style(iced::Color::from_rgb(0.0, 0.0, 0.0))
        };

        let time_display = container(time_text)
            .center_x()
            .center_y()
            .padding(20);

        let start_button = button("Start")
            .on_press(Message::Start)
            .padding(10)
            .style(theme::Button::Secondary);

        let stop_button = button("Stop")
            .on_press(Message::Stop)
            .padding(10)
            .style(theme::Button::Secondary);

        let reset_button = button("Reset")
            .on_press(Message::Reset)
            .padding(10)
            .style(theme::Button::Secondary);

        let controls = row![start_button, stop_button, reset_button]
            .spacing(10)
            .padding(10)
            .align_items(Alignment::Center);

        let content = column![time_display, controls]
            .spacing(20)
            .align_items(Alignment::Center)
            .padding(20);

        container(content)
            .width(Length::Shrink)
            .height(Length::Shrink)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        if self.is_running {
            iced::time::every(std::time::Duration::from_millis(100))
                .map(|_| Message::Tick)
        } else {
            iced::Subscription::none()
        }
    }
}
