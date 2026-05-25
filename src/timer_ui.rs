use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Application, Color, Command, Element, Length, Theme, theme};
use std::time::Instant;

use crate::timer_cfg::load_config;

pub struct TimerApp {
    elapsed_time: std::time::Duration,
    is_running: bool,
    start_time: Option<Instant>,
    paused_time: std::time::Duration,
    limit_for_red_color_in_seconds: u64,
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Reset,
    Tick,
}

impl Application for TimerApp {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
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
        // Color palette - modern teal accent with neutral grays
        let accent_color = Color::from_rgb(0.06, 0.85, 0.91);
        let text_primary = Color::from_rgb(0.09, 0.11, 0.15);
        let text_secondary = Color::from_rgb(0.45, 0.50, 0.62);

        let minutes = self.elapsed_time.as_secs() / 60;
        let seconds = self.elapsed_time.as_secs() % 60;
        let display = format!("{:02}:{:02}", minutes, seconds);

        let time_text = text(display)
            .size(80)
            .style(if self.elapsed_time.as_secs() >= self.limit_for_red_color_in_seconds {
                theme::Text::Color(Color::from_rgb(0.84, 0.16, 0.16))
            } else {
                theme::Text::Color(text_primary)
            });

        let time_display = container(time_text)
            .center_x()
            .center_y()
            .padding([32, 36])
            .style(theme::Container::Box);

        // Status indicator - running dot with color change
        let status_label = if self.is_running { "● Running" } else { "○ Paused" };
        let status_color = if self.is_running { accent_color } else { text_secondary };

        let status_text = text(status_label)
            .size(14)
            .style(theme::Text::Color(status_color));

        // Progress bar using containers
        let total_seconds = (self.elapsed_time.as_secs()).max(1) as u16;
        let progress_percent = (total_seconds % 60).min(60) as u16;

        // Build progress bar with filled and empty portions
        let filled_width = if progress_percent > 0 {
            (progress_percent as f32 / 60.0 * 100.0) as u16
        } else {
            1
        };

        let progress_filled = container("")
            .width(iced::Length::FillPortion(filled_width))
            .height(iced::Length::Fixed(4.0))
            .style(theme::Container::Box);

        let empty_portion = (100u16).saturating_sub(filled_width);
        let progress_empty = container("")
            .width(iced::Length::FillPortion(empty_portion.max(1)))
            .height(iced::Length::Fixed(4.0));

        let progress_bar = row![progress_filled, progress_empty]
            .width(Length::Fill)
            .height(iced::Length::Fixed(4.0))
            .spacing(0);

        let start_button = button(
            container(text("Start")).width(Length::Fill).center_x()
        )
            .on_press(Message::Start)
            .padding(14)
            .width(Length::Fill)
            .style(theme::Button::Primary);

        let stop_button = button(
            container(text("Stop")).width(Length::Fill).center_x()
        )
            .on_press(Message::Stop)
            .padding(14)
            .width(Length::Fill)
            .style(theme::Button::Secondary);

        let reset_button = button(
            container(text("Reset")).width(Length::Fill).center_x()
        )
            .on_press(Message::Reset)
            .padding(14)
            .width(Length::Fill)
            .style(theme::Button::Secondary);

        let controls = row![start_button, stop_button, reset_button]
            .spacing(10)
            .align_items(Alignment::Center)
            .width(Length::Fill);

        let content = column![
            time_display,
            status_text,
            progress_bar,
            controls,
        ]
        .spacing(20)
        .align_items(Alignment::Center)
        .padding(28);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(theme::Container::Box)
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
