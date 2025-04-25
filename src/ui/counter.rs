use iced::{
    widget::{Button, Column, Container, Row, Text},
    Alignment, Element, Task,
};
use iced_aw::sidebar::TabLabel;

use crate::{Message, Tab};

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
        (CounterTab { value: 0 }, Task::none())
    }

    pub fn update(&mut self, message: CounterMessage) -> Task<Message> {
        dbg!(&self.value);
        match message {
            CounterMessage::Increase => self.value += 1,
            CounterMessage::Decrease => self.value -= 1,
        }
        Task::none()
    }
}

impl Tab for CounterTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, CounterMessage> = Container::new(
            Column::new()
                .align_x(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(Text::new(format!("Count: {}", self.value)).size(32))
                .push(
                    Row::new()
                        .spacing(10)
                        .push(Button::new(Text::new("Decrease")).on_press(CounterMessage::Decrease))
                        .push(
                            Button::new(Text::new("Increase")).on_press(CounterMessage::Increase),
                        ),
                ),
        )
        .into();

        content.map(Message::Counter)
    }
}
