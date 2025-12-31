use cosmic::{Action, Element, app::Task};

mod create;
mod detail;
pub mod list;

use detail::EquipamentDetailPage;
use list::ListEquipamentsTab;

use crate::ui::list_equipaments::create::CreateModelPage;

enum View {
    ListEquipamentModels(ListEquipamentsTab),
    DetailEquipamentModel(EquipamentDetailPage),
    CreateEquipamentModel(create::CreateModelPage),
}

#[derive(Debug, Clone)]
pub enum EquipamentListMessage {
    ListEquipaments(Action<list::UsersMessage>),
    DetailEquipament(Action<detail::EquipamentDetailMessage>),
    CreateEquipament(Action<create::CreateModelMessage>),
    GoBack,
}

pub struct EquipamentListTab {
    view: View,
}

impl EquipamentListTab {
    pub fn new() -> (Self, cosmic::app::Task<EquipamentListMessage>) {
        let (page, task) = ListEquipamentsTab::init();
        (
            Self {
                view: View::ListEquipamentModels(page),
            },
            task.map(|msg| Action::App(EquipamentListMessage::ListEquipaments(msg))),
        )
    }
    pub fn update(&mut self, message: EquipamentListMessage) -> Task<EquipamentListMessage> {
        match message {
            EquipamentListMessage::ListEquipaments(action) => {
                if let View::ListEquipamentModels(list_tab) = &mut self.view {
                    match list_tab.update(action) {
                        Action::App(list::UsersMessage::GoToDetail(user)) => {
                            let (page, task) = EquipamentDetailPage::init(user);
                            self.view = View::DetailEquipamentModel(page);
                            task.map(|msg| {
                                Action::App(EquipamentListMessage::DetailEquipament(msg))
                            })
                        }
                        Action::App(list::UsersMessage::CreateEquipament) => {
                            let (page, task) = CreateModelPage::new();
                            self.view = View::CreateEquipamentModel(page);
                            task.map(|msg| {
                                Action::App(EquipamentListMessage::CreateEquipament(msg))
                            })
                        }
                        _ => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            EquipamentListMessage::DetailEquipament(action) => {
                if let View::DetailEquipamentModel(list_tab) = &mut self.view {
                    if let Action::App(detail::EquipamentDetailMessage::Close) = &action {
                        let (page, task) = ListEquipamentsTab::init();
                        self.view = View::ListEquipamentModels(page);

                        return task
                            .map(|msg| Action::App(EquipamentListMessage::ListEquipaments(msg)));
                    }
                    let _ = list_tab.update(action);
                }

                Task::none()
            }
            EquipamentListMessage::CreateEquipament(action) => {
                if let View::CreateEquipamentModel(list_tab) = &mut self.view {
                    if let Action::App(create::CreateModelMessage::CreatedUser(arg)) = &action {
                        if *arg {
                            let (page, task) = ListEquipamentsTab::init();
                            self.view = View::ListEquipamentModels(page);
                            return task.map(|msg| {
                                Action::App(EquipamentListMessage::ListEquipaments(msg))
                            });
                        }
                    }
                    let task = list_tab.update(action);
                    return task
                        .map(|msg| Action::App(EquipamentListMessage::CreateEquipament(msg)));
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
                .map(|msg| EquipamentListMessage::ListEquipaments(Action::App(msg))),

            View::DetailEquipamentModel(page) => page
                .view()
                .map(|msg| EquipamentListMessage::DetailEquipament(Action::App(msg))),
            View::CreateEquipamentModel(page) => page
                .view()
                .map(|msg| EquipamentListMessage::CreateEquipament(Action::App(msg))),
        }
    }
}
