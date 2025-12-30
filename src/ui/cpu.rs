use std::collections::HashMap;

use cosmic::iced::{self, Alignment, Background, Color, Length};
use cosmic::widget::{
    self as widget, button, column, combo_box, container, row, scrollable, table, text_input,
};
use cosmic::{Action, Apply, Element};

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
    DeleteCPU,
    CPUDeleted(bool),
    NoOp,
    ItemSelect(table::Entity),
    OpenCreateModal,
    CloseCreateModal,
}

pub struct CPUsTab {
    cpus: table::SingleSelectModel<Item, Category>,
    cpu: String,
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

impl CPUsTab {
    pub fn new() -> (Self, Task<CPUsMessage>) {
        let table_model = table::Model::new(vec![Category::Name]);
        let app = Self {
            cpus: table_model,
            cpu: String::new(),
            brand: Some(String::new()),
            brands: combo_box::State::new(Vec::new()),
            show_create_modal: false,
        };
        let command = Task::perform(database::get_cpus(), |arg| {
            let tmp = arg.unwrap_or_default();
            Action::App(CPUsMessage::GetCPUs(tmp))
        });
        (app, command)
    }

    pub fn update(&mut self, message: CPUsMessage) -> Task<CPUsMessage> {
        match message {
            CPUsMessage::GetCPUs(cpus) => {
                let mut table_mode = table::Model::new(vec![Category::Name, Category::Brand]);

                cpus.into_iter().for_each(|i| {
                    let tmp = Item {
                        name: i.name,
                        brand: i.brand,
                    };
                    let _ = table_mode.insert(tmp);
                });
                self.cpus = table_mode;
            }
            CPUsMessage::ItemSelect(entity) => self.cpus.activate(entity),

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
                return command;
            }
            CPUsMessage::DeleteCPU => {
                let cpu = self
                    .cpus
                    .item(self.cpus.active())
                    .cloned()
                    .unwrap_or_default()
                    .name;

                let command = Task::perform(database::delete_cpu(cpu.clone()), |arg| {
                    let result = match arg {
                        Ok(_) => true,
                        Err(_) => false,
                    };
                    Action::App(CPUsMessage::CPUDeleted(result))
                });
                return command;
            }
            CPUsMessage::CPUDeleted(deleteado) => {
                if deleteado {
                    let command = Task::perform(database::get_cpus(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(CPUsMessage::GetCPUs(tmp))
                    });
                    return command;
                }
            }
            CPUsMessage::CPUCreated(criado) => {
                if criado {
                    let command = Task::perform(database::get_cpus(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(CPUsMessage::GetCPUs(tmp))
                    });
                    self.show_create_modal = false;
                    return command;
                }
            }
            CPUsMessage::ChangingName(name) => {
                self.cpu = name;
            }
            CPUsMessage::NoOp => todo!(),

            CPUsMessage::LoadedBrands(departs) => {
                let mut tmp = Vec::new();
                departs.iter().for_each(|f| tmp.push(f.name.clone()));
                self.brands = combo_box::State::new(tmp);
            }

            CPUsMessage::GetBrands => {
                let command = Task::perform(database::get_brands(), |arg| {
                    let tmp = arg.unwrap_or_default();
                    Action::App(CPUsMessage::LoadedBrands(tmp))
                });
                return command;
            }

            CPUsMessage::ChagingBrands(actual) => {
                self.brand = Some(actual);
            }
            CPUsMessage::OpenCreateModal => {
                self.show_create_modal = true;
            }
            CPUsMessage::CloseCreateModal => {
                self.show_create_modal = false;
            }
        };
        Task::none()
    }

    pub fn view(&self) -> Element<'_, CPUsMessage> {
        let cpu_delete = button::text("criar cpu").on_press(CPUsMessage::OpenCreateModal);

        let create_cpu: Element<'_, CPUsMessage> = row().push(cpu_delete).into();

        let cpu_list = widget::table(&self.cpus)
            .on_item_left_click(CPUsMessage::ItemSelect)
            .item_context(|item| {
                Some(widget::menu::items(
                    &HashMap::new(),
                    vec![widget::menu::Item::Button(
                        format!("Excluir CPU on {}", item.name),
                        None,
                        MyAction::Selecionado,
                    )],
                ))
            })
            .on_item_right_click(CPUsMessage::ItemSelect)
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

        let scrol = scrollable(cpu_list);

        let content = column()
            .push(create_cpu)
            .push(column().push(scrol).spacing(8))
            .spacing(16)
            .padding(20)
            .max_width(600);

        let base = container(content)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        if self.show_create_modal {
            let cpu_input = text_input("CPU name", &self.cpu).on_input(CPUsMessage::ChangingName);

            let brand_input = combo_box(
                &self.brands,
                "Select brand",
                self.brand.as_ref(),
                CPUsMessage::ChagingBrands,
            )
            .on_open(CPUsMessage::GetBrands);

            let actions = row()
                .push(button::text("Cancel").on_press(CPUsMessage::CloseCreateModal))
                .push(button::text("Create").on_press(CPUsMessage::CreateCPU))
                .spacing(8);

            let modal_content = container(
                column()
                    .push(cpu_input)
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
                .on_close(CPUsMessage::CloseCreateModal);

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
    type Message = CPUsMessage;

    fn message(&self) -> Self::Message {
        CPUsMessage::DeleteCPU
    }
}
