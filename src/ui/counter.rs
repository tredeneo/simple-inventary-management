use cosmic::{
    Element, Task,
    iced::{
        Alignment, Length,
        widget::{button, column, row},
    },
    widget::{container, text},
};

use crate::Message;

#[derive(Debug, Clone)]
pub enum CounterMessage {
    Increase,
    Decrease,
}

#[derive(Default)]
pub struct CounterTab {
    value: i32,
}

impl CounterTab {
    pub fn new() -> (Self, Task<CounterMessage>) {
        (Self { value: 0 }, Task::none())
    }

    pub fn update(&mut self, message: CounterMessage) -> cosmic::app::Task<Message> {
        match message {
            CounterMessage::Increase => self.value += 1,
            CounterMessage::Decrease => self.value -= 1,
        }
        cosmic::app::Task::none()
    }

    // pub fn view(&self) -> Element<'_, Message> {
    pub fn view(&self) -> Element<'_, CounterMessage> {
        let count_text = text(format!("Count: {}", self.value)).size(32);

        let buttons = row![
            button("Decrease").on_press(CounterMessage::Decrease),
            button("Increase").on_press(CounterMessage::Increase),
        ]
        .spacing(10);

        let content = column![count_text, buttons]
            .spacing(16)
            .padding(20)
            .max_width(600);

        // .align_items(Alignment::Center);

        // let tmp = container(content).width(Length::Fill);
        // let tmp: Element<'_, CounterMessage> =
        container(content)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
        // tmp
        // tmp.map(Message::Counter)
    }
}
