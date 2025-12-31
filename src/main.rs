#![cfg_attr(
    all(target_os = "windows", build_release),
    windows_subsystem = "windows"
)]

use simple_inventary::{
    Message, database,
    ui::{
        brand::BrandsTab, cpu::CPUsTab, equipament_models::EquipamentModelsTab, gpu::GPUsTab,
        list_equipaments::EquipamentListTab, list_users::UsersTab,
    },
};

use cosmic::{
    Action, Application, ApplicationExt, Element,
    app::{Core, Settings, Task},
    executor,
    iced::{self, Alignment, Length},
    widget::{container, nav_bar, text},
};

use simple_inventary::ui::departments::DepartmentsTab;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    ListUsers,
    Departments,
    Brands,
    Cpu,
    Gpu,
    EquipamentModels,
    ListEquipaments,
}

impl Page {
    fn title(&self) -> &'static str {
        match self {
            Page::ListUsers => "Users",
            Page::Departments => "Departments",
            Page::Brands => "Brands",
            Page::Cpu => "CPUs",
            Page::Gpu => "GPUs",
            Page::EquipamentModels => "Equipament Models",
            Page::ListEquipaments => "Equipamentos",
        }
    }
}

pub struct App {
    core: Core,
    nav_model: nav_bar::Model,
    users_tab: UsersTab,
    departments_tab: DepartmentsTab,
    brands_tab: BrandsTab,
    cpu_tab: CPUsTab,
    gpu_tab: GPUsTab,
    equipament_models_tab: EquipamentModelsTab,
    equipaments_tab: EquipamentListTab,
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

        nav_model.insert().text("Usuários").data(Page::ListUsers);
        nav_model
            .insert()
            .text("Departamentos")
            .data(Page::Departments);
        nav_model.insert().text("Marcas").data(Page::Brands);
        nav_model.insert().text("CPUs").data(Page::Cpu);
        nav_model.insert().text("GPUs").data(Page::Gpu);
        nav_model
            .insert()
            .text("Equipament Models")
            .data(Page::EquipamentModels);
        nav_model
            .insert()
            .text("Equipaments")
            .data(Page::ListEquipaments);
        nav_model.activate_position(0);
        let (users_page, task) = UsersTab::init();

        (
            Self {
                core,
                nav_model,
                users_tab: users_page,
                departments_tab: DepartmentsTab::new().0,
                brands_tab: BrandsTab::new().0,
                cpu_tab: CPUsTab::new().0,
                gpu_tab: GPUsTab::new().0,
                equipament_models_tab: EquipamentModelsTab::new().0,
                equipaments_tab: EquipamentListTab::new().0,
            },
            task.map(|msg| Action::App(Message::Users(msg))),
        )
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav_model)
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Self::Message> {
        if let Some(page) = self.nav_model.data::<Page>(id).copied() {
            self.nav_model.activate(id);
            match page {
                Page::ListUsers => {
                    let (screen, task) = UsersTab::init();
                    self.users_tab = screen;
                    return task.map(|msg| Action::App(Message::Users(msg)));
                }
                Page::Departments => {
                    let (screen, task) = DepartmentsTab::new();
                    self.departments_tab = screen;
                    return task.map(|msg| Action::App(Message::Departments(msg)));
                }
                Page::Brands => {
                    let (screen, task) = BrandsTab::new();
                    self.brands_tab = screen;
                    return task.map(|msg| Action::App(Message::Brands(msg)));
                }
                Page::Cpu => {
                    let (screen, task) = CPUsTab::new();
                    self.cpu_tab = screen;
                    return task.map(|msg| Action::App(Message::Cpus(msg)));
                }
                Page::Gpu => {
                    let (screen, task) = GPUsTab::new();
                    self.gpu_tab = screen;
                    return task.map(|msg| Action::App(Message::Gpus(msg)));
                }
                Page::EquipamentModels => {
                    let (screen, task) = EquipamentModelsTab::new();
                    self.equipament_models_tab = screen;
                    return task.map(|msg| Action::App(Message::EquipamentModels(msg)));
                }
                Page::ListEquipaments => {
                    let (screen, task) = EquipamentListTab::new();
                    self.equipaments_tab = screen;
                    return task.map(|msg| Action::App(Message::Equipaments(msg)));
                }
            }
        };

        self.update_title()
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::Users(msg) => {
                if let Action::App(inner_msg) = msg {
                    let task = self.users_tab.update(inner_msg);
                    return task.map(|msg| Action::App(Message::Users(msg)));
                }
            }
            Message::Departments(msg) => {
                if let Action::App(inner_msg) = msg {
                    let task = self.departments_tab.update(inner_msg);
                    return task.map(|msg| Action::App(Message::Departments(msg)));
                }
            }
            Message::Brands(msg) => {
                if let Action::App(inner_msg) = msg {
                    let task = self.brands_tab.update(inner_msg);
                    return task.map(|msg| Action::App(Message::Brands(msg)));
                }
            }
            Message::Cpus(msg) => {
                if let Action::App(inner_msg) = msg {
                    let task = self.cpu_tab.update(inner_msg);
                    return task.map(|msg| Action::App(Message::Cpus(msg)));
                }
            }
            Message::Gpus(msg) => {
                if let Action::App(inner_msg) = msg {
                    let task = self.gpu_tab.update(inner_msg);
                    return task.map(|msg| Action::App(Message::Gpus(msg)));
                }
            }
            Message::EquipamentModels(msg) => {
                if let Action::App(inner_msg) = msg {
                    let task = self.equipament_models_tab.update(inner_msg);
                    return task.map(|msg| Action::App(Message::EquipamentModels(msg)));
                }
            }
            Message::Equipaments(msg) => {
                if let Action::App(inner_msg) = msg {
                    let task = self.equipaments_tab.update(inner_msg);
                    return task.map(|msg| Action::App(Message::Equipaments(msg)));
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content = match self.nav_model.active_data::<Page>().copied() {
            Some(Page::ListUsers) => self
                .users_tab
                .view()
                .map(|arg| Message::Users(Action::App(arg))),
            Some(Page::Departments) => self
                .departments_tab
                .view()
                .map(|arg| Message::Departments(Action::App(arg))),
            Some(Page::Brands) => self
                .brands_tab
                .view()
                .map(|arg| Message::Brands(Action::App(arg))),
            Some(Page::Cpu) => self
                .cpu_tab
                .view()
                .map(|arg| Message::Cpus(Action::App(arg))),
            Some(Page::Gpu) => self
                .gpu_tab
                .view()
                .map(|arg| Message::Gpus(Action::App(arg))),
            Some(Page::EquipamentModels) => self
                .equipament_models_tab
                .view()
                .map(|arg| Message::EquipamentModels(Action::App(arg))),
            Some(Page::ListEquipaments) => self
                .equipaments_tab
                .view()
                .map(|arg| Message::Equipaments(Action::App(arg))),
            None => container(text("Nenhuma aba ativa")).into(),
        };

        container(content)
            .width(Length::Fill)
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
            self.set_window_title(title, win_id)
        } else {
            Task::none()
        }
    }
}

fn main() -> iced::Result {
    let _ = database::init_database();
    let settings = Settings::default()
        .antialiasing(true)
        .client_decorations(true)
        .debug(cfg!(debug_assertions))
        .default_icon_theme("Pop")
        .default_text_size(16.0)
        .scale_factor(1.0)
        .size(cosmic::iced_core::Size::new(1280.0, 720.0));

    cosmic::app::run::<App>(settings, ())
}
