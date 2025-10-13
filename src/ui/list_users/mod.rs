use cosmic::{Action, Element, app::Task};

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
    GoBack,
}

pub struct UsersTab {
    view: View,
}

impl UsersTab {
    pub fn init() -> (Self, cosmic::app::Task<UsersTabMessage>) {
        let (page, task) = ListUserTab::init();
        (
            Self {
                view: View::ListUsers(page),
            },
            task.map(|msg| Action::App(UsersTabMessage::ListUsers(msg))),
        )
    }
    pub fn update(&mut self, message: UsersTabMessage) -> Task<UsersTabMessage> {
        match message {
            UsersTabMessage::ListUsers(action) => {
                if let View::ListUsers(list_tab) = &mut self.view {
                    match list_tab.update(action) {
                        Action::App(list::UsersMessage::GoToDetail(user)) => {
                            let (page, task) = UserDetailPage::init(user);
                            self.view = View::DetailUser(page);
                            task.map(|msg| Action::App(UsersTabMessage::DetailUser(msg)))
                        }
                        _ => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            UsersTabMessage::DetailUser(action) => {
                if let View::DetailUser(list_tab) = &mut self.view {
                    match action {
                        Action::App(detail::UserDetailMessage::Close) => {
                            let (page, task) = ListUserTab::init();
                            self.view = View::ListUsers(page);

                            return task.map(|msg| Action::App(UsersTabMessage::ListUsers(msg)));
                        }
                        tmp => {
                            let _ = list_tab.update(tmp);
                        }
                    }
                }

                Task::none()
            }
            _ => Task::none(),
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
