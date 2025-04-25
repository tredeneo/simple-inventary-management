#![cfg_attr(
    all(target_os = "windows", build_release),
    windows_subsystem = "windows"
)]

use iced::{Element, Task};
use iced_aw::sidebar::SidebarWithContent;

use iced_async_screen_example::ui::counter::CounterTab;

use iced_async_screen_example::ui::brand::{TestAsyncAction, TestAsyncTab};
use iced_async_screen_example::{Message, Tab, TabId};

use iced_async_screen_example::ui::grid::{self, GridTestTab};

fn main() -> iced::Result {
    iced::application(
        "Sidebar example",
        TabBarExample::update,
        TabBarExample::view,
    )
    .run()
}

#[derive(Default)]
struct TabBarExample {
    active_tab: TabId,
    counter_tab: CounterTab,
    lista_tab: TestAsyncTab,
    grid_tab: grid::GridTestTab,
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
            Message::AsyncTest(message) => {
                let action = self.lista_tab.update(message);
                match action {
                    TestAsyncAction::None => Task::none(),
                    TestAsyncAction::Run(task) => task.map(Message::AsyncTest),
                }
            }
            Message::GridTest(message) => {
                let _ = self.grid_tab.update(message);
                Task::none()
            }
        }
    }
    fn reset_tab(&mut self, tab_id: TabId) {
        match tab_id {
            TabId::Counter => self.counter_tab = CounterTab::new().0,
            TabId::Lista => self.lista_tab = TestAsyncTab::new().0,
            TabId::GridTest => self.grid_tab = GridTestTab::new().0,
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
