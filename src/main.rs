#![cfg_attr(
    all(target_os = "windows", build_release),
    windows_subsystem = "windows"
)]

use iced::{Element, Task};
use iced_aw::sidebar::SidebarWithContent;

use simple_inventary::ui::counter::CounterTab;

use simple_inventary::{Message, Tab, TabId};

use simple_inventary::ui::list_users;
use simple_inventary::ui::list_users::{ListUsers, ListUsersAction};

fn main() -> iced::Result {
    iced::application(
        "Sidebar example",
        TabBarExample::update,
        TabBarExample::view,
    )
    .run()
}

// #[derive(Default)]
struct TabBarExample {
    active_tab: TabId,
    counter_tab: CounterTab,
    grid_tab: list_users::ListUsers,
}

impl Default for TabBarExample {
    fn default() -> Self {
        Self {
            active_tab: TabId::Counter,
            counter_tab: CounterTab::new().0,
            grid_tab: ListUsers::new().0,
        }
    }
}

impl TabBarExample {
    fn update(&mut self, message: Message) -> Task<Message> {
        dbg!(&message);
        match message {
            Message::TabSelected(selected) => {
                self.active_tab = selected.clone();
                self.initialize_screens(selected)
            }
            Message::Counter(message) => {
                let _ = self.counter_tab.update(message);
                Task::none()
            }

            Message::ListUsers(message) => match self.grid_tab.update(message) {
                ListUsersAction::None => Task::none(),
                ListUsersAction::Run(task) => task.map(Message::ListUsers),
            },
        }
    }

    fn initialize_screens(&mut self, tab_id: TabId) -> Task<Message> {
        match tab_id {
            TabId::Counter => {
                self.counter_tab = CounterTab::new().0;
                Task::none()
            }

            TabId::ListUsers => {
                let (screen, task) = ListUsers::new();
                self.grid_tab = screen;

                task.map(Message::ListUsers)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        SidebarWithContent::new(Message::TabSelected)
            .tab_icon_position(iced_aw::sidebar::Position::End)
            .push(
                TabId::Counter,
                self.counter_tab.tab_label(),
                self.counter_tab.view(),
            )
            .push(
                TabId::ListUsers,
                self.grid_tab.tab_label(),
                self.grid_tab.view(),
            )
            .set_active_tab(&self.active_tab)
            .sidebar_position(iced_aw::sidebar::SidebarPosition::Start)
            .into()
    }
}
