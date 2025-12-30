use cosmic::app::Task;
use cosmic::iced::widget::button;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::text;
use cosmic::{Action, Element};

use crate::database;

pub struct UserDetailPage {
    name: String,
}

#[derive(Debug, Clone)]
pub enum UserDetailMessage {
    Close,
    Save,
    GetUserDetail(database::model::DbUser),
}

impl UserDetailPage {
    pub fn init(user: String) -> (Self, Task<UserDetailMessage>) {
        let app = UserDetailPage {
            name: String::from("nome teste"),
        };

        let command = Task::perform(database::get_specific_user_by_name(user), |arg| {
            let tmp = arg.unwrap_or_default();
            cosmic::Action::App(UserDetailMessage::GetUserDetail(tmp))
        });
        (app, command)
    }

    pub fn update(&mut self, message: Action<UserDetailMessage>) -> Action<UserDetailMessage> {
        match message {
            Action::App(message) => match message {
                UserDetailMessage::GetUserDetail(user) => {
                    self.name = user.name;
                    Action::None
                }
                UserDetailMessage::Close => Action::None,
                _ => Action::None,
            },
            _ => Action::None,
        }
    }
    pub fn view(&self) -> Element<'_, UserDetailMessage> {
        use cosmic::iced::widget::{column, row};
        use cosmic::widget::container;
        let buttons = row![button("back").on_press(UserDetailMessage::Close)];
        let coluna = column![buttons, text(format!("{}", self.name)).size(32),];
        container(coluna)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }
}
