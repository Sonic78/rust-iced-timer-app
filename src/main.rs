use iced::widget::text;
use iced::{Element, Sandbox};

fn main() -> iced::Result {
    TimerApp::run(iced::Settings::default())
}

struct TimerApp;

#[derive(Debug)]
enum Message {}

impl Sandbox for TimerApp {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Cool Iced Timer")
    }

    fn update(&mut self, message: Message) {
        match message {
            // Handle messages here
        }
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello, world!").into()
    }    
}
