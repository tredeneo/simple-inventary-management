use cosmic::{
    Action, Element,
    iced::{
        Alignment, Length,
        widget::{column, row},
    },
    iced_widget::text_input,
    widget::{combo_box, container, text},
};

use crate::database;
use cosmic::app::Task;

#[derive(Debug, Clone)]
pub enum CPUsMessage {
    LoadedBrands(Vec<database::model::DbBrand>),
    GetBrands,
    ChagingBrands(String),
    GetCPUs(Vec<database::model::DbCPU>),
    CreateCPU,
    ChangingName(String),
    CPUCreated(bool),
}

#[derive(Default)]
pub struct CPUsTab {
    cpus: Vec<database::model::DbCPU>,
    cpu: String,
    brand: Option<String>,
    brands: combo_box::State<String>,
}

impl CPUsTab {
    pub fn new() -> (Self, Task<CPUsMessage>) {
        let app = Self {
            cpus: Vec::new(),
            cpu: String::new(),
            brand: Some(String::new()),
            brands: combo_box::State::new(Vec::new()),
        };
        let command = Task::perform(database::get_cpus(), |cpus| {
            let cpus = cpus.unwrap_or_default();
            Action::App(CPUsMessage::GetCPUs(cpus))
        });
        (app, command)
    }

    pub fn update(&mut self, message: CPUsMessage) -> Task<CPUsMessage> {
        match message {
            CPUsMessage::GetCPUs(cpus) => {
                self.cpus = cpus;
                Task::none()
            }
            CPUsMessage::LoadedBrands(departs) => {
                let mut tmp = Vec::new();
                departs.iter().for_each(|f| tmp.push(f.name.clone()));
                self.brands = combo_box::State::new(tmp);
                Task::none()
            }
            CPUsMessage::CreateCPU => {
                let command = Task::perform(
                    database::insert_cpu(self.cpu.clone(), self.brand.clone().unwrap_or_default()),
                    |arg| {
                        let result = match arg {
                            Ok(_) => true,
                            Err(_) => false,
                        };
                        Action::App(CPUsMessage::CPUCreated(result))
                    },
                );
                command
            }
            CPUsMessage::CPUCreated(criado) => {
                if criado {
                    let command = Task::perform(database::get_cpus(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(CPUsMessage::GetCPUs(tmp))
                    });
                    return command;
                }

                return Task::none();
            }
            CPUsMessage::ChangingName(name) => {
                self.cpu = name;
                Task::none()
            }
            CPUsMessage::ChagingBrands(actual) => {
                self.brand = Some(actual);
                Task::none()
            }
            CPUsMessage::GetBrands => {
                let command = Task::perform(database::get_brands(), |arg| {
                    let tmp = arg.unwrap_or_default();
                    Action::App(CPUsMessage::LoadedBrands(tmp))
                });
                return command;
            }
        }
    }

    pub fn view(&self) -> Element<'_, CPUsMessage> {
        let brand = combo_box(
            &self.brands,
            "Selecione a marca",
            self.brand.as_ref(),
            CPUsMessage::ChagingBrands,
        )
        .on_open(CPUsMessage::GetBrands);
        let cpu_text = text_input("Nova Marca", &self.cpu)
            .on_input(CPUsMessage::ChangingName)
            .on_submit(CPUsMessage::CreateCPU);
        let create_cpu: Element<'_, CPUsMessage> = row![cpu_text, brand].into();
        let cpu_list: Vec<Element<'_, CPUsMessage>> = self
            .cpus
            .iter()
            .map(|cpu| Element::from(text(format!("marca:{}", cpu.name))))
            .collect();
        let content = column![create_cpu]
            .push(column(cpu_list).spacing(8))
            .spacing(16)
            .padding(20)
            .max_width(600);

        container(content)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }
}
