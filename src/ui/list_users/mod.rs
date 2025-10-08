use cosmic::{Action, Element};

mod detail;
pub mod list;

use detail::UserDetailPage;
use list::ListUserTab;

enum View {
    ListUsers(ListUserTab),
    DetailUser(UserDetailPage),
}

#[derive(Debug, Clone)]
pub enum UsersTabMessage {
    ListUsers(Action<list::UsersMessage>),
    DetailUser(Action<detail::UserDetailMessage>),
    GoToDetail(String),
    GoBack,
}

pub struct UsersTab {
    view: View,
}

impl UsersTab {
    pub fn init() -> (Self, cosmic::app::Task<UsersTabMessage>) {
        let (list_tab, task) = ListUserTab::init();
        (
            Self {
                view: View::ListUsers(list_tab),
            },
            task.map(|msg| Action::App(UsersTabMessage::ListUsers(msg))), // cosmic::app::Task::none(),
                                                                          // task.map(|msg| (UsersTabMessage::ListUsers(msg))), // cosmic::app::Task::none(),
                                                                          // task.map(|msg| Action::App(UsersTabMessage::ListUsers(Action::App(msg)))), // cosmic::app::Task::none(),
                                                                          // task.map(|msg| Action::App(UsersTabMessage::ListUsers(msg))),
        )
    }
    pub fn update(&mut self, message: UsersTabMessage) {
        match message {
            UsersTabMessage::GoToDetail(user_name) => {
                let (page, _) = UserDetailPage::init();
                self.view = View::DetailUser(page);
            }
            UsersTabMessage::GoBack => {
                let (list_tab, _) = ListUserTab::init();

                self.view = View::ListUsers(list_tab);
            }
            UsersTabMessage::ListUsers(action) => {
                if let View::ListUsers(list_tab) = &mut self.view {
                    match &action {
                        Action::App(list::UsersMessage::ItemSelect(_entity)) => {
                            let (page, _) = UserDetailPage::init();
                            self.view = View::DetailUser(page);
                        }
                        _ => {
                            let _ = list_tab.update(action);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    pub fn view(&self) -> Element<'_, UsersTabMessage> {
        match &self.view {
            View::ListUsers(list_tab) => list_tab
                .view()
                .map(|msg| UsersTabMessage::ListUsers(Action::App(msg))),

            View::DetailUser(page) => page
                .view()
                .map(|msg| UsersTabMessage::DetailUser(Action::App(msg))),
        }
    }
}
