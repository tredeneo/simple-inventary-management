use cosmic::Apply;
use std::collections::HashMap;

use cosmic::app::Task;
use cosmic::iced::widget::button;
use cosmic::iced::{self, Alignment, Length};
use cosmic::widget::{self, table, text};
use cosmic::{Action, Element};

use crate::database;

pub struct EquipamentDetailPage {
    serial: String,
    model: String,
    memory: i32,
    storage: i32,
    observation: String,
    users: table::SingleSelectModel<Item, Category>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Category {
    #[default]
    Name,
    DataBegin,
    DataEnd,
}

#[derive(Default, Clone, Debug)]
struct Item {
    name: String,
    data_begin: String,
    data_end: String,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Name => "Name",
            Self::DataBegin => "Data Begin",
            Self::DataEnd => "Data End",
        })
    }
}

impl table::ItemCategory for Category {
    fn width(&self) -> iced::Length {
        match self {
            Self::Name => iced::Length::Fixed(400.0),
            Self::DataBegin => iced::Length::Fixed(200.0),
            Self::DataEnd => iced::Length::Fill,
        }
    }
}

impl table::ItemInterface<Category> for Item {
    fn get_icon(&self, _: Category) -> Option<cosmic::widget::Icon> {
        None
    }

    fn get_text(&self, category: Category) -> std::borrow::Cow<'static, str> {
        match category {
            Category::Name => self.name.clone().into(),
            Category::DataBegin => self.data_begin.clone().into(),
            Category::DataEnd => self.data_end.clone().into(),
        }
    }

    fn compare(&self, other: &Self, category: Category) -> std::cmp::Ordering {
        match category {
            Category::Name => self.name.to_lowercase().cmp(&other.name.to_lowercase()),
            Category::DataBegin => self
                .data_begin
                .to_lowercase()
                .cmp(&other.data_begin.to_lowercase()),
            Category::DataEnd => self
                .data_end
                .to_lowercase()
                .cmp(&other.data_end.to_lowercase()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EquipamentDetailMessage {
    Close,
    Save,
    InfoUpdated(
        (
            Vec<database::model::DbLastUser>,
            database::model::DbComputer,
        ),
    ),

    ItemSelect(table::Entity),
    CategorySelect(Category),
    NoOp,
}

impl EquipamentDetailPage {
    pub fn init(equipament: String) -> (Self, Task<EquipamentDetailMessage>) {
        let tmp = table::Model::new(vec![Category::Name, Category::DataBegin, Category::DataEnd]);
        let app = EquipamentDetailPage {
            serial: String::new(),
            storage: 0,
            memory: 0,
            model: String::new(),
            observation: String::new(),
            users: tmp,
        };

        let command = Task::perform(
            async move {
                (
                    database::get_user_computers(&equipament).await,
                    database::get_specific_computer(&equipament).await,
                )
            },
            |(user, computer)| {
                let user = user.unwrap_or_default();
                let computer = computer.unwrap_or_default();
                cosmic::Action::App(EquipamentDetailMessage::InfoUpdated((user, computer)))
            },
        );
        (app, command)
    }

    pub fn update(
        &mut self,
        message: Action<EquipamentDetailMessage>,
    ) -> Action<EquipamentDetailMessage> {
        match message {
            Action::App(message) => match message {
                EquipamentDetailMessage::InfoUpdated((users, computer)) => {
                    let mut table_users = table::Model::new(vec![
                        Category::Name,
                        Category::DataBegin,
                        Category::DataEnd,
                    ]);
                    users.iter().for_each(|i| {
                        let tmp = Item {
                            name: i.usuario.clone(),
                            data_begin: i.date_begin.clone(),
                            data_end: i.date_end.clone().unwrap_or_default(),
                        };
                        let _ = table_users.insert(tmp);
                    });
                    self.serial = computer.serialnumber;
                    self.model = computer.model;
                    self.memory = computer.memory;
                    self.observation = computer.observation;
                    self.storage = computer.storage;
                    self.users = table_users;
                    Action::None
                }
                EquipamentDetailMessage::Close => Action::None,

                EquipamentDetailMessage::ItemSelect(entity) => {
                    self.users.activate(entity);
                    Action::None
                }
                EquipamentDetailMessage::CategorySelect(category) => {
                    let mut ascending = true;
                    if let Some(old_sort) = self.users.get_sort()
                        && old_sort.0 == category
                    {
                        ascending = !old_sort.1;
                    }
                    self.users.sort(category, ascending);
                    Action::None
                }
                _ => Action::None,
            },

            _ => Action::None,
        }
    }
    fn ui_table(&self) -> Element<'_, EquipamentDetailMessage> {
        let table_widget = widget::table(&self.users)
            // .on_item_left_click(UsersMessage::ItemSelect)
            .on_category_left_click(EquipamentDetailMessage::CategorySelect)
            .item_context(|item| {
                Some(widget::menu::items(
                    &HashMap::new(),
                    vec![widget::menu::Item::Button(
                        format!("Action on {}", item.name),
                        None,
                        MyAction::None,
                    )],
                ))
            })
            .category_context(|category| {
                Some(widget::menu::items(
                    &HashMap::new(),
                    vec![widget::menu::Item::Button(
                        format!("Other action on {} category", category.to_string()),
                        None,
                        MyAction::None,
                    )],
                ))
            })
            .apply(Element::from);
        table_widget
    }

    pub fn view(&self) -> Element<'_, EquipamentDetailMessage> {
        use cosmic::iced::widget::{column, row};
        use cosmic::widget::container;
        let buttons = row![button("back").on_press(EquipamentDetailMessage::Close)];
        let coluna = column![
            buttons,
            text(format!(" teste {}", self.model)).size(32),
            self.ui_table()
        ];

        container(coluna)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MyAction {
    None,
}

impl widget::menu::Action for MyAction {
    type Message = EquipamentDetailMessage;

    fn message(&self) -> Self::Message {
        EquipamentDetailMessage::NoOp
    }
}
