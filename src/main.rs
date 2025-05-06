#![cfg_attr(
    all(target_os = "windows", build_release),
    windows_subsystem = "windows"
)]

use simple_inventary::Message;

use cosmic::{
    Application, ApplicationExt, Element,
    app::{Core, Settings, Task},
    executor,
    iced::{self, Alignment, Length},
    widget::{container, nav_bar, text},
};

use simple_inventary::ui::counter::CounterTab;
// use simple_inventary::ui::list_users::UsersTab;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Counter,
    // UsersTab,
}

impl Page {
    fn title(&self) -> &'static str {
        match self {
            Page::Counter => "Contador",
            // Page::UsersTab => "Usuários",
        }
    }
}

// #[derive(Debug, Clone)]
// pub enum Message {
//     Counter(simple_inventary::Message),
//     // ListUsers(simple_inventary::ui::list_users::ListUsersMessage),
// }

pub struct App {
    core: Core,
    nav_model: nav_bar::Model,
    counter_tab: CounterTab,
    // list_users: UsersTab,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "org.simple_inventory.CosmicApp";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut nav_model = nav_bar::Model::default();

        nav_model.insert().text("Contador").data(Page::Counter);
        // nav_model.insert().text("Usuários").data(Page::UsersTab);
        nav_model.activate_position(0);

        (
            Self {
                core,
                nav_model,
                counter_tab: CounterTab::new().0,
                // list_users: UsersTab::new().0,
            },
            Task::none(),
        )
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav_model)
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Self::Message> {
        if let Some(page) = self.nav_model.data::<Page>(id).copied() {
            match page {
                Page::Counter => {
                    self.counter_tab = CounterTab::new().0;
                } // Page::UsersTab => {
                  //     let (screen, _task) = UsersTab::new();
                  //     self.list_users = screen;
                  // }
            }
            self.nav_model.activate(id);
        }

        self.update_title()
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::Counter(msg) => {
                let _ = self.counter_tab.update(msg);
                Task::none()
            } // Message::ListUsers(msg) => self
              //     .list_users
              //     .update(msg)
              //     .map(|task| task.map(Message::ListUsers))
              //     .unwrap_or(Task::none()),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let content = match self.nav_model.active_data::<Page>().copied() {
            Some(Page::Counter) => self.counter_tab.view().map(|msg| Message::Counter(msg)),
            // Some(Page::UsersTab) => self.list_users.view().map(Message::ListUsers),
            None => container(text("Nenhuma aba ativa")).into(),
        };

        container(content)
            .width(Length::Fill)
            // .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }
}

impl App {
    fn active_page_title(&self) -> &str {
        self.nav_model
            .active_data::<Page>()
            .map(Page::title)
            .unwrap_or("Sem título")
    }

    fn update_title(&mut self) -> Task<Message> {
        let title = format!("{} — Inventário", self.active_page_title());

        if let Some(win_id) = self.core.main_window_id() {
            // self.set_window_title(title, win_id)
            self.set_window_title(title)
        } else {
            Task::none()
        }
    }
}

fn main() -> iced::Result {
    let settings = Settings::default()
        .antialiasing(true)
        .client_decorations(true)
        .debug(cfg!(debug_assertions))
        .default_icon_theme("Pop")
        .default_text_size(16.0)
        .scale_factor(1.0)
        .size(cosmic::iced_core::Size::new(1024.0, 768.0));

    cosmic::app::run::<App>(settings, ())
}
