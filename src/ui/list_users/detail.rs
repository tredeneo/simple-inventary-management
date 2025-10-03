use cosmic::Element;
use cosmic::app::Task;
use cosmic::iced::widget::button;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::text;

use crate::database;

pub struct UserDetailPage {
    name: String,
}

#[derive(Clone)]
enum UserDetailMessage {
    Close,
    Save,
    GetUserDetail(Vec<database::model::DbUser>),
}

impl UserDetailPage {
    pub fn init() -> (Self, Task<UserDetailMessage>) {
        let app = UserDetailPage {
            name: String::from("nome teste"),
        };

        let command = Task::perform(database::get_users(), |arg| {
            let tmp = arg.unwrap_or_default();
            cosmic::Action::App(UserDetailMessage::GetUserDetail(tmp))
        });
        (app, command)
    }
    pub fn view(&self) -> Element<'_, UserDetailMessage> {
        use cosmic::iced::widget::{column, row};
        use cosmic::widget::container;
        let buttons = row![button("back").on_press(UserDetailMessage::Close)];
        let coluna = column![
            buttons,
            text(format!("{}", self.name)).size(32),
            // text(tmp.department).size(30),
            // text(tmp.email).size(30)
        ];
        container(coluna)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }
}
