mod detail;
mod list;

use iced::Element;
use iced_aw::sidebar::TabLabel;

pub struct UsersTab {
    screen: list::ListUsers,
}

#[derive(Debug, Clone)]
pub enum UsersMessage {
    List(list::ListUsersMessage),
    // Futuramente: Detail(...)
}

impl UsersTab {
    pub fn new() -> (Self, iced::Task<UsersMessage>) {
        let (screen, task) = list::ListUsers::new();
        (Self { screen }, task.map(UsersMessage::List))
    }

    pub fn update(&mut self, message: UsersMessage) -> Option<iced::Task<UsersMessage>> {
        match message {
            UsersMessage::List(msg) => match self.screen.update(msg) {
                list::Action::None => None,
                list::Action::Run(task) => Some(task.map(UsersMessage::List)),
            },
        }
    }

    pub fn view(&self) -> Element<'_, UsersMessage> {
        self.screen.view().map(UsersMessage::List)
    }

    pub fn title(&self) -> String {
        "Users List".into()
    }

    pub fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }
}
