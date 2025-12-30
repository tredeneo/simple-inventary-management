use cosmic::{
    iced::{
        widget::{column, row},
        Alignment, Length,
    },
    iced_widget::text_input,
    widget::{combo_box, container, text},
    Action, Element,
};

use crate::database;
use cosmic::app::Task;

#[derive(Debug, Clone)]
pub enum GPUsMessage {
    LoadedBrands(Vec<database::model::DbBrand>),
    GetBrands,
    ChagingBrands(String),
    GetGPUs(Vec<database::model::DbGPU>),
    CreateGPU,
    ChangingName(String),
    GPUCreated(bool),
}

#[derive(Default)]
pub struct GPUsTab {
    gpus: Vec<database::model::DbGPU>,
    gpu: String,
    brand: Option<String>,
    brands: combo_box::State<String>,
}

impl GPUsTab {
    pub fn new() -> (Self, Task<GPUsMessage>) {
        let app = Self {
            gpus: Vec::new(),
            gpu: String::new(),
            brand: Some(String::new()),
            brands: combo_box::State::new(Vec::new()),
        };
        let command = Task::perform(database::get_gpus(), |gpus| {
            let gpus = gpus.unwrap_or_default();
            Action::App(GPUsMessage::GetGPUs(gpus))
        });
        (app, command)
    }

    pub fn update(&mut self, message: GPUsMessage) -> Task<GPUsMessage> {
        match message {
            GPUsMessage::GetGPUs(gpus) => {
                self.gpus = gpus;
                Task::none()
            }
            GPUsMessage::LoadedBrands(departs) => {
                let mut tmp = Vec::new();
                departs.iter().for_each(|f| tmp.push(f.name.clone()));
                self.brands = combo_box::State::new(tmp);
                Task::none()
            }
            GPUsMessage::CreateGPU => {
                let command = Task::perform(
                    database::insert_gpu(self.gpu.clone(), self.brand.clone().unwrap_or_default()),
                    |arg| {
                        let result = match arg {
                            Ok(_) => true,
                            Err(_) => false,
                        };
                        Action::App(GPUsMessage::GPUCreated(result))
                    },
                );
                command
            }
            GPUsMessage::GPUCreated(criado) => {
                if criado {
                    let command = Task::perform(database::get_gpus(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(GPUsMessage::GetGPUs(tmp))
                    });
                    return command;
                }

                return Task::none();
            }
            GPUsMessage::ChangingName(name) => {
                self.gpu = name;
                Task::none()
            }
            GPUsMessage::ChagingBrands(actual) => {
                self.brand = Some(actual);
                Task::none()
            }
            GPUsMessage::GetBrands => {
                let command = Task::perform(database::get_brands(), |arg| {
                    let tmp = arg.unwrap_or_default();
                    Action::App(GPUsMessage::LoadedBrands(tmp))
                });
                return command;
            }
        }
    }

    pub fn view(&self) -> Element<'_, GPUsMessage> {
        let brand = combo_box(
            &self.brands,
            "Selecione a marca",
            self.brand.as_ref(),
            GPUsMessage::ChagingBrands,
        )
        .on_open(GPUsMessage::GetBrands);
        let gpu_text = text_input("Nova Marca", &self.gpu)
            .on_input(GPUsMessage::ChangingName)
            .on_submit(GPUsMessage::CreateGPU);
        let create_gpu: Element<'_, GPUsMessage> = row![gpu_text, brand].into();
        let gpu_list: Vec<Element<'_, GPUsMessage>> = self
            .gpus
            .iter()
            .map(|gpu| Element::from(text(format!("marca:{}", gpu.name))))
            .collect();
        let content = column![create_gpu]
            .push(column(gpu_list).spacing(8))
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
