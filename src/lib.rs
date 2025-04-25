pub mod ui;

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Column, Container, Text},
    Element, Length,
};
use iced_aw::sidebar::TabLabel;

use crate::ui::brand::TestAsyncMessage;

use crate::ui::counter::CounterMessage;

use crate::ui::grid::GridMessage;
#[derive(Clone, Debug)]
pub enum Message {
    TabSelected(TabId),
    Counter(CounterMessage),
    AsyncTest(TestAsyncMessage),
    GridTest(GridMessage),
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum TabId {
    #[default]
    Counter,
    Lista,
    GridTest,
}

use serde::{Deserialize, Serialize};

pub type Welcome = Vec<WelcomeElement>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WelcomeElement {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub email: String,
    pub address: Address,
    pub phone: String,
    pub website: String,
    pub company: Company,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    pub geo: Geo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Geo {
    pub lat: String,
    pub lng: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub name: String,
    pub catch_phrase: String,
    pub bs: String,
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

