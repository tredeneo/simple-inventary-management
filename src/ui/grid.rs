use iced::{widget::text, Task};
use iced::{Color, Element, Length};
use iced_aw::{grid, grid_row};

use crate::Message;
use crate::Tab;

#[derive(Default)]
pub struct GridTestTab {}

#[derive(Debug, Clone)]
pub enum GridMessage {}

impl GridTestTab {
    pub fn new() -> (Self, Task<GridMessage>) {
        (Self {}, Task::none())
    }
    pub fn update(&mut self, _message: GridMessage) -> Task<GridMessage> {
        Task::none()
    }
}
impl Tab for GridTestTab {
    fn content(&self) -> iced::Element<'_, Self::Message> {
        let primeiro = grid_row!(
            text("first"),
            text("first first"),
            text("first first first"),
            text("first first first first"),
        );
        let segundo = grid_row!(
            text("second"),
            text("second second"),
            text("second second second"),
            text("second second second second"),
        );
        let terceiro = grid_row!(
            text("thirt"),
            text("thirt thirt"),
            text("thirt thirt thirt"),
            text("thirt thirt thirt thirst"),
        );
        let content: Element<'_, GridMessage> = grid!(primeiro, segundo, terceiro)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .into();

        let content: Element<'_, GridMessage> = content.explain(Color::from_rgb(255.0, 0.0, 0.0));
        content.map(Message::GridTest)
    }

    type Message = Message;

    fn title(&self) -> String {
        String::from("Grid")
    }

    fn tab_label(&self) -> iced_aw::sidebar::TabLabel {
        iced_aw::sidebar::TabLabel::Text(self.title())
    }
}
