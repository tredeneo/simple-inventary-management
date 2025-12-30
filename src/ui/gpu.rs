use std::collections::HashMap;

use cosmic::iced::{self, Alignment, Background, Color, Length};
use cosmic::widget::{
    self as widget, button, column, combo_box, container, row, scrollable, table, text_input,
};
use cosmic::{Action, Apply, Element};

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
    DeleteGPU,
    GPUDeleted(bool),
    NoOp,
    ItemSelect(table::Entity),
    OpenCreateModal,
    CloseCreateModal,
}

pub struct GPUsTab {
    gpus: table::SingleSelectModel<Item, Category>,
    gpu: String,
    brand: Option<String>,
    brands: combo_box::State<String>,
    show_create_modal: bool,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
enum Category {
    #[default]
    Name,
    Brand,
}

#[derive(Default, Debug, Clone)]
struct Item {
    name: String,
    brand: String,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Name => "Name",
            Self::Brand => "Brand",
        })
    }
}

impl table::ItemCategory for Category {
    fn width(&self) -> iced::Length {
        match self {
            Self::Name => iced::Length::Fixed(300.0),
            Self::Brand => iced::Length::Fill,
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
        }
    }

    fn compare(&self, other: &Self, category: Category) -> std::cmp::Ordering {
        match category {
            Category::Name => self.name.to_lowercase().cmp(&other.name.to_lowercase()),
            Category::Brand => self.brand.to_lowercase().cmp(&other.name.to_lowercase()),
        }
    }
}

impl GPUsTab {
    pub fn new() -> (Self, Task<GPUsMessage>) {
        let table_model = table::Model::new(vec![Category::Name]);
        let app = Self {
            gpus: table_model,
            gpu: String::new(),
            brand: Some(String::new()),
            brands: combo_box::State::new(Vec::new()),
            show_create_modal: false,
        };
        let command = Task::perform(database::get_gpus(), |arg| {
            let tmp = arg.unwrap_or_default();
            Action::App(GPUsMessage::GetGPUs(tmp))
        });
        (app, command)
    }

    pub fn update(&mut self, message: GPUsMessage) -> Task<GPUsMessage> {
        match message {
            GPUsMessage::GetGPUs(gpus) => {
                let mut table_mode = table::Model::new(vec![Category::Name, Category::Brand]);

                gpus.into_iter().for_each(|i| {
                    let tmp = Item {
                        name: i.name,
                        brand: i.brand,
                    };
                    let _ = table_mode.insert(tmp);
                });
                self.gpus = table_mode;
            }
            GPUsMessage::ItemSelect(entity) => self.gpus.activate(entity),

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
                return command;
            }
            GPUsMessage::DeleteGPU => {
                let gpu = self
                    .gpus
                    .item(self.gpus.active())
                    .cloned()
                    .unwrap_or_default()
                    .name;

                let command = Task::perform(database::delete_gpu(gpu.clone()), |arg| {
                    let result = match arg {
                        Ok(_) => true,
                        Err(_) => false,
                    };
                    Action::App(GPUsMessage::GPUDeleted(result))
                });
                return command;
            }
            GPUsMessage::GPUDeleted(deleteado) => {
                if deleteado {
                    let command = Task::perform(database::get_gpus(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(GPUsMessage::GetGPUs(tmp))
                    });
                    return command;
                }
            }
            GPUsMessage::GPUCreated(criado) => {
                if criado {
                    let command = Task::perform(database::get_gpus(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(GPUsMessage::GetGPUs(tmp))
                    });
                    self.show_create_modal = false;
                    return command;
                }
            }
            GPUsMessage::ChangingName(name) => {
                self.gpu = name;
            }
            GPUsMessage::NoOp => todo!(),

            GPUsMessage::LoadedBrands(departs) => {
                let mut tmp = Vec::new();
                departs.iter().for_each(|f| tmp.push(f.name.clone()));
                self.brands = combo_box::State::new(tmp);
            }

            GPUsMessage::GetBrands => {
                let command = Task::perform(database::get_brands(), |arg| {
                    let tmp = arg.unwrap_or_default();
                    Action::App(GPUsMessage::LoadedBrands(tmp))
                });
                return command;
            }

            GPUsMessage::ChagingBrands(actual) => {
                self.brand = Some(actual);
            }
            GPUsMessage::OpenCreateModal => {
                self.show_create_modal = true;
            }
            GPUsMessage::CloseCreateModal => {
                self.show_create_modal = false;
            }
        };
        Task::none()
    }

    pub fn view(&self) -> Element<'_, GPUsMessage> {
        let gpu_delete = button::text("criar gpu").on_press(GPUsMessage::OpenCreateModal);

        let create_gpu: Element<'_, GPUsMessage> = row().push(gpu_delete).into();

        let gpu_list = widget::table(&self.gpus)
            .on_item_left_click(GPUsMessage::ItemSelect)
            .item_context(|item| {
                Some(widget::menu::items(
                    &HashMap::new(),
                    vec![widget::menu::Item::Button(
                        format!("Excluir GPU on {}", item.name),
                        None,
                        MyAction::Selecionado,
                    )],
                ))
            })
            .on_item_right_click(GPUsMessage::ItemSelect)
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

        let scrol = scrollable(gpu_list);

        let content = column()
            .push(create_gpu)
            .push(column().push(scrol).spacing(8))
            .spacing(16)
            .padding(20)
            .max_width(600);

        let base = container(content)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        if self.show_create_modal {
            let gpu_input = text_input("GPU name", &self.gpu).on_input(GPUsMessage::ChangingName);

            let brand_input = combo_box(
                &self.brands,
                "Select brand",
                self.brand.as_ref(),
                GPUsMessage::ChagingBrands,
            )
            .on_open(GPUsMessage::GetBrands);

            let actions = row()
                .push(button::text("Cancel").on_press(GPUsMessage::CloseCreateModal))
                .push(button::text("Create").on_press(GPUsMessage::CreateGPU))
                .spacing(8);

            let modal_content = container(
                column()
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
                .on_close(GPUsMessage::CloseCreateModal);

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
    type Message = GPUsMessage;

    fn message(&self) -> Self::Message {
        GPUsMessage::DeleteGPU
    }
}
