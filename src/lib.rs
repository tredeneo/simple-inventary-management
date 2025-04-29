pub mod database;
pub mod ui;

use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{Column, Container, Text},
};
use iced_aw::sidebar::TabLabel;

use crate::ui::counter::CounterMessage;

use crate::ui::list_users::UsersMessage;
#[derive(Clone, Debug)]
pub enum Message {
    TabSelected(TabId),
    Counter(CounterMessage),
    ListUsers(UsersMessage),
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum TabId {
    #[default]
    Counter,
    UsersTab,
}

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;
pub trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content())
            .align_x(iced::Alignment::Center);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}
