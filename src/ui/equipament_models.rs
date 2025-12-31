use std::collections::HashMap;

use cosmic::iced::{self, Alignment, Background, Color, Length};
use cosmic::widget::{
    self as widget, button, column, combo_box, container, row, scrollable, table, text_input,
};
use cosmic::{Action, Apply, Element};

use crate::database;
use crate::database::model::DbEquipamentModel;
use cosmic::app::Task;

#[derive(Debug, Clone)]
pub enum EquipamentModelsMessage {
    LoadedBrands(Vec<database::model::DbBrand>),
    GetBrands,
    ChagingBrands(String),

    LoadedCPU(Vec<database::model::DbCPU>),
    GetCPUs,
    ChagingCPU(String),

    LoadedGPUs(Vec<database::model::DbGPU>),
    GetGPUs,
    ChagingGPU(String),

    GetEquipamentModels(Vec<database::model::DbEquipamentModel>),
    CreateEquipamentModel,
    ChangingName(String),
    EquipamentModelCreated(bool),
    DeleteEquipamentModel,
    EquipamentModelDeleted(bool),
    ItemSelect(table::Entity),
    OpenCreateModal,
    CloseCreateModal,
}

pub struct EquipamentModelsTab {
    equipament_models: table::SingleSelectModel<Item, Category>,
    equipament_model: String,
    brand: Option<String>,
    brands: combo_box::State<String>,
    cpu: Option<String>,
    cpus: combo_box::State<String>,
    gpu: Option<String>,
    gpus: combo_box::State<String>,
    show_create_modal: bool,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
enum Category {
    #[default]
    Name,
    Brand,
    CPU,
    GPU,
    SmartPhone,
}

#[derive(Default, Debug, Clone)]
struct Item {
    name: String,
    brand: String,
    cpu: String,
    gpu: String,
    smarthphone: String,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Name => "Name",
            Self::Brand => "Brand",
            Self::CPU => "CPU",
            Self::GPU => "GPU",
            Self::SmartPhone => "SmartPhone",
        })
    }
}

impl table::ItemCategory for Category {
    fn width(&self) -> iced::Length {
        match self {
            Self::Name => iced::Length::Fixed(200.0),
            Self::Brand => iced::Length::Fixed(100.0),
            Self::CPU => iced::Length::Fixed(200.0),
            Self::GPU => iced::Length::Fixed(200.0),
            Self::SmartPhone => iced::Length::Fill,
        }
    }
}

impl table::ItemInterface<Category> for Item {
    fn get_icon(&self, _category: Category) -> Option<cosmic::widget::Icon> {
        None
    }

    fn get_text(&self, category: Category) -> std::borrow::Cow<'static, str> {
        match category {
            Category::Name => self.name.clone().into(),
            Category::Brand => self.brand.clone().into(),
            Category::CPU => self.cpu.clone().into(),
            Category::GPU => self.gpu.clone().into(),
            Category::SmartPhone => self.smarthphone.clone().into(),
        }
    }

    fn compare(&self, other: &Self, category: Category) -> std::cmp::Ordering {
        match category {
            Category::Name => self.name.to_lowercase().cmp(&other.name.to_lowercase()),
            Category::CPU => self.cpu.to_lowercase().cmp(&other.name.to_lowercase()),
            Category::GPU => self.gpu.to_lowercase().cmp(&other.name.to_lowercase()),
            Category::SmartPhone => self
                .smarthphone
                .to_lowercase()
                .cmp(&other.name.to_lowercase()),
            Category::Brand => self.brand.to_lowercase().cmp(&other.name.to_lowercase()),
        }
    }
}

impl EquipamentModelsTab {
    pub fn new() -> (Self, Task<EquipamentModelsMessage>) {
        let table_model = table::Model::new(vec![Category::Name]);
        let app = Self {
            equipament_models: table_model,
            equipament_model: String::new(),
            brand: Some(String::new()),
            brands: combo_box::State::new(Vec::new()),
            cpu: Some(String::new()),
            cpus: combo_box::State::new(Vec::new()),
            gpu: Some(String::new()),
            gpus: combo_box::State::new(Vec::new()),
            show_create_modal: false,
        };
        let command = Task::perform(database::get_equipament_model(), |arg| {
            let tmp = arg.unwrap_or_default();
            Action::App(EquipamentModelsMessage::GetEquipamentModels(tmp))
        });
        (app, command)
    }

    pub fn update(&mut self, message: EquipamentModelsMessage) -> Task<EquipamentModelsMessage> {
        match message {
            EquipamentModelsMessage::GetEquipamentModels(equipament_models) => {
                let mut table_mode = table::Model::new(vec![
                    Category::Name,
                    Category::Brand,
                    Category::CPU,
                    Category::GPU,
                    Category::SmartPhone,
                ]);

                equipament_models.into_iter().for_each(|i| {
                    let tmp = Item {
                        name: i.name,
                        brand: i.brand,
                        cpu: i.cpu,
                        gpu: i.gpu,
                        smarthphone: i.smartphone.to_string(),
                    };
                    let _ = table_mode.insert(tmp);
                });
                self.equipament_models = table_mode;
            }
            EquipamentModelsMessage::ItemSelect(entity) => self.equipament_models.activate(entity),

            EquipamentModelsMessage::CreateEquipamentModel => {
                let command = Task::perform(
                    {
                        let tmp = DbEquipamentModel {
                            name: self.equipament_model.clone(),
                            brand: self.brand.clone().unwrap_or_default(),
                            cpu: self.equipament_model.clone(),
                            gpu: self.equipament_model.clone(),
                            smartphone: 1,
                        };
                        database::insert_equipament_model(tmp)
                    },
                    |arg| {
                        let result = match arg {
                            Ok(_) => true,
                            Err(_) => false,
                        };
                        Action::App(EquipamentModelsMessage::EquipamentModelCreated(result))
                    },
                );
                return command;
            }
            EquipamentModelsMessage::DeleteEquipamentModel => {
                let equipament_model = self
                    .equipament_models
                    .item(self.equipament_models.active())
                    .cloned()
                    .unwrap_or_default()
                    .name;

                let command = Task::perform(
                    database::delete_equipament_model(equipament_model.clone()),
                    |arg| {
                        let result = match arg {
                            Ok(_) => true,
                            Err(_) => false,
                        };
                        Action::App(EquipamentModelsMessage::EquipamentModelDeleted(result))
                    },
                );
                return command;
            }
            EquipamentModelsMessage::EquipamentModelDeleted(deleteado) => {
                if deleteado {
                    let command = Task::perform(database::get_equipament_model(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(EquipamentModelsMessage::GetEquipamentModels(tmp))
                    });
                    return command;
                }
            }
            EquipamentModelsMessage::EquipamentModelCreated(criado) => {
                if criado {
                    let command = Task::perform(database::get_equipament_model(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(EquipamentModelsMessage::GetEquipamentModels(tmp))
                    });
                    self.show_create_modal = false;
                    return command;
                }
            }
            EquipamentModelsMessage::ChangingName(name) => {
                self.equipament_model = name;
            }

            EquipamentModelsMessage::LoadedBrands(departs) => {
                let mut tmp = Vec::new();
                departs.iter().for_each(|f| tmp.push(f.name.clone()));
                self.brands = combo_box::State::new(tmp);
            }
            EquipamentModelsMessage::LoadedCPU(departs) => {
                let mut tmp = Vec::new();
                departs.iter().for_each(|f| tmp.push(f.name.clone()));
                self.cpus = combo_box::State::new(tmp);
            }
            EquipamentModelsMessage::LoadedGPUs(departs) => {
                let mut tmp = Vec::new();
                departs.iter().for_each(|f| tmp.push(f.name.clone()));
                self.gpus = combo_box::State::new(tmp);
            }
            EquipamentModelsMessage::GetBrands => {
                let command = Task::perform(database::get_brands(), |arg| {
                    let tmp = arg.unwrap_or_default();
                    Action::App(EquipamentModelsMessage::LoadedBrands(tmp))
                });
                return command;
            }

            EquipamentModelsMessage::GetCPUs => {
                let command = Task::perform(database::get_cpus(), |arg| {
                    let tmp = arg.unwrap_or_default();
                    Action::App(EquipamentModelsMessage::LoadedCPU(tmp))
                });
                return command;
            }

            EquipamentModelsMessage::GetGPUs => {
                let command = Task::perform(database::get_gpus(), |arg| {
                    let tmp = arg.unwrap_or_default();
                    Action::App(EquipamentModelsMessage::LoadedGPUs(tmp))
                });
                return command;
            }
            EquipamentModelsMessage::ChagingBrands(actual) => self.brand = Some(actual),
            EquipamentModelsMessage::ChagingCPU(actual) => self.cpu = Some(actual),
            EquipamentModelsMessage::ChagingGPU(actual) => self.gpu = Some(actual),

            EquipamentModelsMessage::OpenCreateModal => {
                self.show_create_modal = true;
            }
            EquipamentModelsMessage::CloseCreateModal => {
                self.show_create_modal = false;
            }
        };
        Task::none()
    }

    pub fn view(&self) -> Element<'_, EquipamentModelsMessage> {
        let equipament_model_delete = button::text("criar equipament model")
            .on_press(EquipamentModelsMessage::OpenCreateModal);

        let create_equipament_model: Element<'_, EquipamentModelsMessage> =
            row().push(equipament_model_delete).into();

        let equipament_model_list = widget::table(&self.equipament_models)
            .on_item_left_click(EquipamentModelsMessage::ItemSelect)
            .item_context(|item| {
                Some(widget::menu::items(
                    &HashMap::new(),
                    vec![widget::menu::Item::Button(
                        format!("Excluir {}", item.name),
                        None,
                        MyAction::Selecionado,
                    )],
                ))
            })
            .on_item_right_click(EquipamentModelsMessage::ItemSelect)
            .category_context(|_| {
                Some(widget::menu::items(
                    &HashMap::new(),
                    vec![widget::menu::Item::Button(
                        "".to_string(),
                        None,
                        MyAction::Selecionado,
                    )],
                ))
            })
            .apply(Element::from);

        let scrol = scrollable(equipament_model_list);

        let content = column()
            .push(create_equipament_model)
            .push(column().push(scrol).spacing(8))
            .spacing(16)
            .padding(20)
            .max_width(900);

        let base = container(content)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        if self.show_create_modal {
            let equipament_model_input = text_input("EquipamentModel name", &self.equipament_model)
                .on_input(EquipamentModelsMessage::ChangingName);

            let brand_input = combo_box(
                &self.brands,
                "Select brand",
                self.brand.as_ref(),
                EquipamentModelsMessage::ChagingBrands,
            )
            .on_open(EquipamentModelsMessage::GetBrands);

            let cpu_input = combo_box(
                &self.cpus,
                "Select cpu",
                self.cpu.as_ref(),
                EquipamentModelsMessage::ChagingCPU,
            )
            .on_open(EquipamentModelsMessage::GetCPUs);

            let gpu_input = combo_box(
                &self.gpus,
                "Select brand",
                self.gpu.as_ref(),
                EquipamentModelsMessage::ChagingGPU,
            )
            .on_open(EquipamentModelsMessage::GetGPUs);

            let actions = row()
                .push(button::text("Cancel").on_press(EquipamentModelsMessage::CloseCreateModal))
                .push(
                    button::text("Create").on_press(EquipamentModelsMessage::CreateEquipamentModel),
                )
                .spacing(8);

            let modal_content = container(
                column()
                    .push(equipament_model_input)
                    .push(cpu_input)
                    .push(gpu_input)
                    .push(brand_input)
                    .push(actions)
                    .spacing(12)
                    .padding(20)
                    .width(Length::Fixed(400.0)),
            )
            .style(|theme: &cosmic::Theme| {
                let cosmic = theme.cosmic();
                iced::widget::container::Style {
                    background: Some(Background::Color(Color::from(cosmic.primary.base))),
                    text_color: Some(Color::from(cosmic.primary.on)),
                    ..Default::default()
                }
            });

            let tmp = widget::popover(base)
                .modal(true)
                .position(widget::popover::Position::Center)
                .on_close(EquipamentModelsMessage::CloseCreateModal);

            return tmp.popup(modal_content).into();
        }

        base.into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MyAction {
    Selecionado,
}

impl widget::menu::Action for MyAction {
    type Message = EquipamentModelsMessage;

    fn message(&self) -> Self::Message {
        EquipamentModelsMessage::DeleteEquipamentModel
    }
}
