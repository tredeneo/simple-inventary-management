use cosmic::{Action, Element, app::Task};

mod create;
mod detail;
pub mod list;

use detail::UserDetailPage;
use list::ListUserTab;

use crate::ui::list_users::create::CreateUserPage;

enum View {
    ListUsers(ListUserTab),
    DetailUser(UserDetailPage),
    CreateUser(create::CreateUserPage),
}

#[derive(Debug, Clone)]
pub enum UsersTabMessage {
    ListUsers(Action<list::UsersMessage>),
    DetailUser(Action<detail::UserDetailMessage>),
    CreateUser(Action<create::CreateUserMessage>),
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
                        Action::App(list::UsersMessage::CreateUser) => {
                            let (page, task) = CreateUserPage::new();
                            self.view = View::CreateUser(page);
                            task.map(|msg| Action::App(UsersTabMessage::CreateUser(msg)))
                        }
                        _ => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            UsersTabMessage::DetailUser(action) => {
                if let View::DetailUser(list_tab) = &mut self.view {
                    if let Action::App(detail::UserDetailMessage::Close) = &action {
                        let (page, task) = ListUserTab::init();
                        self.view = View::ListUsers(page);

                        return task.map(|msg| Action::App(UsersTabMessage::ListUsers(msg)));
                    }
                    let _ = list_tab.update(action);
                }

                Task::none()
            }
            UsersTabMessage::CreateUser(action) => {
                if let View::CreateUser(list_tab) = &mut self.view {
                    if let Action::App(create::CreateUserMessage::CreatedUser(arg)) = &action {
                        if *arg {
                            let (page, task) = ListUserTab::init();
                            self.view = View::ListUsers(page);
                            return task.map(|msg| Action::App(UsersTabMessage::ListUsers(msg)));
                        }
                    }
                    let (_, task) = list_tab.update(action);
                    return task.map(|msg| Action::App(UsersTabMessage::CreateUser(msg)));
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
            View::CreateUser(page) => page
                .view()
                .map(|msg| UsersTabMessage::CreateUser(Action::App(msg))),
        }
    }
}
