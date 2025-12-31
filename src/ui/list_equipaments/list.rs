use std::collections::HashMap;
use std::fmt::Debug;

use cosmic::Action;
use cosmic::app::Task;
use cosmic::iced::widget::button;
use cosmic::iced::{self, Size};
use cosmic::iced_widget::{column, row};
use cosmic::prelude::*;
use cosmic::widget::{self, container, text_input};
use cosmic::widget::{nav_bar, scrollable, table};

use crate::database;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Category {
    #[default]
    SerialNumber,
    Model,
    ActualUser,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::SerialNumber => "SerialNumber",
            Self::Model => "Departamento",
            Self::ActualUser => "ActualUser",
        })
    }
}

impl table::ItemCategory for Category {
    fn width(&self) -> iced::Length {
        match self {
            Self::SerialNumber => iced::Length::Fixed(400.0),
            Self::Model => iced::Length::Fixed(200.0),
            Self::ActualUser => iced::Length::Fill,
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Item {
    serial_number: String,
    model: String,
    actual_user: String,
}

impl table::ItemInterface<Category> for Item {
    fn get_icon(&self, _: Category) -> Option<cosmic::widget::Icon> {
        None
    }

    fn get_text(&self, category: Category) -> std::borrow::Cow<'static, str> {
        match category {
            Category::SerialNumber => self.serial_number.clone().into(),
            Category::Model => self.model.clone().into(),
            Category::ActualUser => self.actual_user.clone().into(),
        }
    }

    fn compare(&self, other: &Self, category: Category) -> std::cmp::Ordering {
        match category {
            Category::SerialNumber => self
                .serial_number
                .to_lowercase()
                .cmp(&other.serial_number.to_lowercase()),
            Category::Model => self.model.to_lowercase().cmp(&other.model.to_lowercase()),
            Category::ActualUser => self
                .actual_user
                .to_lowercase()
                .cmp(&other.actual_user.to_lowercase()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum UsersMessage {
    ItemSelect(table::Entity),
    CategorySelect(Category),
    GetEquipaments(Vec<database::model::DbComputer>),
    FilterEquipament(String),
    GoToDetail(String),
    CreateEquipamentPressed,
    CreateEquipament,
    NoOp,
}

pub struct ListEquipamentsTab {
    users: Vec<Item>,
    search_field: String,
    table_model: table::SingleSelectModel<Item, Category>,
}

impl ListEquipamentsTab {
    pub fn init() -> (Self, Task<UsersMessage>) {
        let mut nav_model = nav_bar::Model::default();

        nav_model.activate_position(0);

        let table_model = table::Model::new(vec![
            Category::SerialNumber,
            Category::Model,
            Category::ActualUser,
        ]);

        let app = ListEquipamentsTab {
            table_model,
            users: Vec::new(),
            search_field: String::new(),
        };

        let command = Task::perform(database::get_computers(), |arg| {
            let tmp = arg.unwrap_or_default();
            cosmic::Action::App(UsersMessage::GetEquipaments(tmp))
        });

        (app, command)
    }

    pub fn update(&mut self, message: Action<UsersMessage>) -> Action<UsersMessage> {
        match message {
            Action::App(message) => match message {
                UsersMessage::FilterEquipament(filter_user) => {
                    let mut table_model = table::Model::new(vec![
                        Category::SerialNumber,
                        Category::Model,
                        Category::ActualUser,
                    ]);
                    self.users.iter().for_each(|item| {
                        if item.serial_number.contains(&filter_user)
                            || item.actual_user.contains(&filter_user)
                        {
                            let _ = table_model.insert(item.clone());
                        }
                    });
                    self.search_field = filter_user;
                    self.table_model = table_model;

                    Action::None
                }
                UsersMessage::GoToDetail(_user_name) => Action::None,
                UsersMessage::ItemSelect(entity) => {
                    let user = self.table_model.item(entity).cloned().unwrap_or_default();
                    self.table_model.activate(entity);
                    Action::App(UsersMessage::GoToDetail(user.serial_number))
                }
                UsersMessage::CategorySelect(category) => {
                    let mut ascending = true;
                    if let Some(old_sort) = self.table_model.get_sort()
                        && old_sort.0 == category
                    {
                        ascending = !old_sort.1;
                    }
                    self.table_model.sort(category, ascending);
                    Action::None
                }
                UsersMessage::GetEquipaments(db_user) => {
                    let mut table_model = table::Model::new(vec![
                        Category::SerialNumber,
                        Category::Model,
                        Category::ActualUser,
                    ]);
                    db_user.into_iter().for_each(|i| {
                        let tmp = Item {
                            serial_number: i.serialnumber,
                            model: i.model,
                            actual_user: i.actual_user,
                        };
                        self.users.push(tmp.clone());
                        let _ = table_model.insert(tmp);
                    });
                    self.table_model = table_model;

                    Action::None
                }
                UsersMessage::CreateEquipamentPressed => {
                    Action::App(UsersMessage::CreateEquipament)
                }
                UsersMessage::CreateEquipament => Action::None,
                UsersMessage::NoOp => Action::None,
            },
            _ => Action::None,
        }
    }

    fn screen_list_user(&self, size: Size) -> Element<'_, UsersMessage> {
        let search_bar = text_input(&self.search_field, &self.search_field)
            .on_input(UsersMessage::FilterEquipament);
        let create_user = button("criar usuario").on_press(UsersMessage::CreateEquipamentPressed);
        let table_widget = if size.width < 600.0 {
            widget::compact_table(&self.table_model)
                .on_item_left_click(UsersMessage::ItemSelect)
                .apply(Element::from)
        } else {
            widget::table(&self.table_model)
                .on_item_left_click(UsersMessage::ItemSelect)
                .on_category_left_click(UsersMessage::CategorySelect)
                .item_context(|item| {
                    Some(widget::menu::items(
                        &HashMap::new(),
                        vec![widget::menu::Item::Button(
                            format!("Action on {}", item.serial_number),
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
                .apply(Element::from)
        };
        let tmp = row![search_bar, create_user];
        let content = column![tmp, table_widget];

        container(content).into()
    }
    pub fn view(&self) -> Element<'_, UsersMessage> {
        cosmic::widget::responsive(|size| scrollable(self.screen_list_user(size)).into()).into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MyAction {
    None,
}

impl widget::menu::Action for MyAction {
    type Message = UsersMessage;

    fn message(&self) -> Self::Message {
        UsersMessage::NoOp
    }
}
