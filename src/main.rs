#![cfg_attr(
    all(target_os = "windows", build_release),
    windows_subsystem = "windows"
)]

use iced::{Color, Element, Task};
use iced_aw::sidebar::SidebarWithContent;

use simple_inventary::ui::counter::CounterTab;

use simple_inventary::{Message, Tab, TabId};

// use simple_inventary::ui::list_users::ListUsersAction;
use simple_inventary::ui::list_users::UsersTab;

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
    list_users: UsersTab,
}

impl Default for TabBarExample {
    fn default() -> Self {
        Self {
            active_tab: TabId::Counter,
            counter_tab: CounterTab::new().0,
            list_users: UsersTab::new().0,
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

            Message::ListUsers(message) => self
                .list_users
                .update(message)
                .map(|task| task.map(Message::ListUsers))
                .unwrap_or(Task::none()),
        }
    }

    fn initialize_screens(&mut self, tab_id: TabId) -> Task<Message> {
        match tab_id {
            TabId::Counter => {
                self.counter_tab = CounterTab::new().0;
                Task::none()
            }

            TabId::UsersTab => {
                let (screen, task) = UsersTab::new();
                self.list_users = screen;

                task.map(Message::ListUsers)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let tmp: Element<_> = SidebarWithContent::new(Message::TabSelected)
            .tab_icon_position(iced_aw::sidebar::Position::End)
            .push(
                TabId::Counter,
                self.counter_tab.tab_label(),
                self.counter_tab.view(),
            )
            .push(
                TabId::UsersTab,
                self.list_users.tab_label(),
                self.list_users.view().map(Message::ListUsers),
            )
            .set_active_tab(&self.active_tab)
            .sidebar_position(iced_aw::sidebar::SidebarPosition::Start)
            .into();
        if cfg!(debug_assertions) {
            return tmp.explain(Color::from_rgb(255.0, 0.0, 0.0));
        };
        tmp
    }
}
