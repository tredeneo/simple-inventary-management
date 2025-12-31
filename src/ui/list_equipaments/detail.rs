use cosmic::app::Task;
use cosmic::iced::widget::button;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::text;
use cosmic::{Action, Element};

use crate::database;

pub struct EquipamentDetailPage {
    name: String,
}

#[derive(Debug, Clone)]
pub enum EquipamentDetailMessage {
    Close,
    Save,
    GetUserDetail(database::model::DbUser),
}

impl EquipamentDetailPage {
    pub fn init(user: String) -> (Self, Task<EquipamentDetailMessage>) {
        let app = EquipamentDetailPage {
            name: String::from("nome teste"),
        };

        let command = Task::perform(database::get_specific_user_by_name(user), |arg| {
            let tmp = arg.unwrap_or_default();
            cosmic::Action::App(EquipamentDetailMessage::GetUserDetail(tmp))
        });
        (app, command)
    }

    pub fn update(
        &mut self,
        message: Action<EquipamentDetailMessage>,
    ) -> Action<EquipamentDetailMessage> {
        match message {
            Action::App(message) => match message {
                EquipamentDetailMessage::GetUserDetail(user) => {
                    self.name = user.name;
                    Action::None
                }
                EquipamentDetailMessage::Close => Action::None,
                _ => Action::None,
            },
            _ => Action::None,
        }
    }
    pub fn view(&self) -> Element<'_, EquipamentDetailMessage> {
        use cosmic::iced::widget::{column, row};
        use cosmic::widget::container;
        let buttons = row![button("back").on_press(EquipamentDetailMessage::Close)];
        let coluna = column![buttons, text(format!("{}", self.name)).size(32),];
        container(coluna)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }
}
