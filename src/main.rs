#![cfg_attr(
    all(target_os = "windows", build_release),
    windows_subsystem = "windows"
)]

use iced::{Element, Task};
use iced_aw::sidebar::SidebarWithContent;

use simple_inventary::ui::counter::CounterTab;

use simple_inventary::ui::brand::{TestAsyncAction, TestAsyncTab};
use simple_inventary::{Message, Tab, TabId};

use simple_inventary::ui::grid::{self, ListUsers, ListUsersAction};

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
    lista_tab: TestAsyncTab,
    grid_tab: grid::ListUsers,
}

impl Default for TabBarExample {
    fn default() -> Self {
        let tmp = ListUsers::new().0;
        Self {
            active_tab: TabId::Counter,
            counter_tab: CounterTab::new().0,
            lista_tab: TestAsyncTab::new().0,
            grid_tab: tmp,
        }
    }
}

impl TabBarExample {
    fn update(&mut self, message: Message) -> Task<Message> {
        dbg!(&message);
        match message {
            Message::TabSelected(selected) => {
                self.active_tab = selected.clone();
                self.reset_tab(selected);
                Task::none()
            }
            Message::Counter(message) => {
                let _ = self.counter_tab.update(message);
                Task::none()
            }
            Message::AsyncTest(message) => match self.lista_tab.update(message) {
                TestAsyncAction::None => Task::none(),
                TestAsyncAction::Run(task) => task.map(Message::AsyncTest),
            },
            Message::ListUsers(message) => match self.grid_tab.update(message) {
                ListUsersAction::None => Task::none(),
                ListUsersAction::Run(task) => task.map(Message::ListUsers),
            },
        }
    }
    fn reset_tab(&mut self, tab_id: TabId) {
        match tab_id {
            TabId::Counter => {
                self.counter_tab = CounterTab::new().0;
            }
            TabId::Lista => {
                self.lista_tab = TestAsyncTab::new().0;
            }
            TabId::GridTest => {
                self.grid_tab = ListUsers::new().0;
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
                TabId::Lista,
                self.lista_tab.tab_label(),
                self.lista_tab.view(),
            )
            .push(
                TabId::GridTest,
                self.grid_tab.tab_label(),
                self.grid_tab.view(),
            )
            .set_active_tab(&self.active_tab)
            .sidebar_position(iced_aw::sidebar::SidebarPosition::Start)
            .into()
    }
}
