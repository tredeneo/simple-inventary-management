use cosmic::{Action, Element, app::Task};

mod create;
mod detail;
pub mod list;

use detail::UserDetailPage;
use list::ListUserTab;

use crate::ui::list_equipaments::create::CreateModelPage;

enum View {
    ListEquipamentModels(ListUserTab),
    DetailEquipamentModel(UserDetailPage),
    CreateEquipamentModel(create::CreateModelPage),
}

#[derive(Debug, Clone)]
pub enum EquipamentListMessage {
    ListUsers(Action<list::UsersMessage>),
    DetailUser(Action<detail::UserDetailMessage>),
    CreateUser(Action<create::CreateModelMessage>),
    GoBack,
}

pub struct EquipamentListTab {
    view: View,
}

impl EquipamentListTab {
    pub fn init() -> (Self, cosmic::app::Task<EquipamentListMessage>) {
        let (page, task) = ListUserTab::init();
        (
            Self {
                view: View::ListEquipamentModels(page),
            },
            task.map(|msg| Action::App(EquipamentListMessage::ListUsers(msg))),
        )
    }
    pub fn update(&mut self, message: EquipamentListMessage) -> Task<EquipamentListMessage> {
        match message {
            EquipamentListMessage::ListUsers(action) => {
                if let View::ListEquipamentModels(list_tab) = &mut self.view {
                    match list_tab.update(action) {
                        Action::App(list::UsersMessage::GoToDetail(user)) => {
                            let (page, task) = UserDetailPage::init(user);
                            self.view = View::DetailEquipamentModel(page);
                            task.map(|msg| Action::App(EquipamentListMessage::DetailUser(msg)))
                        }
                        Action::App(list::UsersMessage::CreateUser) => {
                            let (page, task) = CreateModelPage::new();
                            self.view = View::CreateEquipamentModel(page);
                            task.map(|msg| Action::App(EquipamentListMessage::CreateUser(msg)))
                        }
                        _ => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            EquipamentListMessage::DetailUser(action) => {
                if let View::DetailEquipamentModel(list_tab) = &mut self.view {
                    if let Action::App(detail::UserDetailMessage::Close) = &action {
                        let (page, task) = ListUserTab::init();
                        self.view = View::ListEquipamentModels(page);

                        return task.map(|msg| Action::App(EquipamentListMessage::ListUsers(msg)));
                    }
                    let _ = list_tab.update(action);
                }

                Task::none()
            }
            EquipamentListMessage::CreateUser(action) => {
                if let View::CreateEquipamentModel(list_tab) = &mut self.view {
                    if let Action::App(create::CreateModelMessage::CreatedUser(arg)) = &action {
                        if *arg {
                            let (page, task) = ListUserTab::init();
                            self.view = View::ListEquipamentModels(page);
                            return task
                                .map(|msg| Action::App(EquipamentListMessage::ListUsers(msg)));
                        }
                    }
                    let task = list_tab.update(action);
                    return task.map(|msg| Action::App(EquipamentListMessage::CreateUser(msg)));
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }
    pub fn view(&self) -> Element<'_, EquipamentListMessage> {
        match &self.view {
            View::ListEquipamentModels(list_tab) => list_tab
                .view()
                .map(|msg| EquipamentListMessage::ListUsers(Action::App(msg))),

            View::DetailEquipamentModel(page) => page
                .view()
                .map(|msg| EquipamentListMessage::DetailUser(Action::App(msg))),
            View::CreateEquipamentModel(page) => page
                .view()
                .map(|msg| EquipamentListMessage::CreateUser(Action::App(msg))),
        }
    }
}
