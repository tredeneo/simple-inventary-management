use std::collections::HashMap;

use cosmic::Action;
use cosmic::app::Task;
use cosmic::iced;
use cosmic::prelude::*;
use cosmic::widget::text;
use cosmic::widget::{self};
use cosmic::widget::{nav_bar, scrollable, table};

use crate::database;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Category {
    #[default]
    Name,
    Department,
    Email,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Name => "Name",
            Self::Department => "Department",
            Self::Email => "Email",
        })
    }
}

impl table::ItemCategory for Category {
    fn width(&self) -> iced::Length {
        match self {
            Self::Name => iced::Length::Fixed(300.0),
            Self::Department => iced::Length::Fixed(200.0),
            Self::Email => iced::Length::Fill,
        }
    }
}

struct Item {
    name: String,
    department: String,
    email: String,
}

impl table::ItemInterface<Category> for Item {
    fn get_icon(&self, category: Category) -> Option<cosmic::widget::Icon> {
        if category == Category::Name {
            Some(cosmic::widget::icon::from_name("application-x-executable-symbolic").icon())
        } else {
            None
        }
    }

    fn get_text(&self, category: Category) -> std::borrow::Cow<'static, str> {
        match category {
            Category::Name => self.name.clone().into(),
            Category::Department => self.department.clone().into(),
            Category::Email => self.email.clone().into(),
        }
    }

    fn compare(&self, other: &Self, category: Category) -> std::cmp::Ordering {
        match category {
            Category::Name => self.name.to_lowercase().cmp(&other.name.to_lowercase()),
            Category::Department => self
                .department
                .to_lowercase()
                .cmp(&other.department.to_lowercase()),
            Category::Email => self.email.to_lowercase().cmp(&other.email.to_lowercase()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum UsersMessage {
    ItemSelect(table::Entity),
    CategorySelect(Category),
    GetUsers(Vec<database::model::DbUser>),
    CloseDetail,
    NoOp,
}

pub struct ListUserTab {
    table_model: table::SingleSelectModel<Item, Category>,
    active_detail: bool,
}

impl ListUserTab {
    pub fn init() -> (Self, Task<UsersMessage>) {
        let mut nav_model = nav_bar::Model::default();

        nav_model.activate_position(0);

        let table_model =
            table::Model::new(vec![Category::Name, Category::Department, Category::Email]);

        let app = ListUserTab {
            table_model,
            active_detail: false,
        };

        let command = Task::perform(database::get_users(), |arg| {
            let tmp = arg.unwrap_or_default();
            cosmic::Action::App(UsersMessage::GetUsers(tmp))
        });

        (app, command)
    }

    pub fn update(&mut self, message: Action<UsersMessage>) -> Action<UsersMessage> {
        match message {
            Action::App(message) => match message {
                UsersMessage::ItemSelect(entity) => {
                    self.table_model.activate(entity);
                    self.active_detail = true;
                    Action::None
                }
                UsersMessage::CategorySelect(category) => {
                    let mut ascending = true;
                    if let Some(old_sort) = self.table_model.get_sort() {
                        if old_sort.0 == category {
                            ascending = !old_sort.1;
                        }
                    }
                    self.table_model.sort(category, ascending);
                    Action::None
                }
                UsersMessage::CloseDetail => {
                    self.active_detail = false;
                    Action::None
                }
                UsersMessage::GetUsers(db_user) => {
                    let mut table_model = table::Model::new(vec![
                        Category::Name,
                        Category::Department,
                        Category::Email,
                    ]);
                    db_user.into_iter().for_each(|i| {
                        let _ = table_model.insert(Item {
                            name: i.name,
                            department: i.department,
                            email: i.email,
                        });
                    });
                    self.table_model = table_model;

                    Action::None
                }
                UsersMessage::NoOp => Action::None,
            },
            _ => Action::None,
        }
    }

    fn screen_detail(&self) -> Element<'_, UsersMessage> {
        let coluna =
            cosmic::iced::widget::column![text("teste 1").size(32), text("teste 2").size(30)];
        coluna.into()
    }
    pub fn view(&self) -> Element<'_, UsersMessage> {
        cosmic::widget::responsive(|size| {
            let table_wdget = if size.width < 600.0 {
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
                                format!("Action on {}", item.name),
                                None,
                                MyAction::None,
                            )],
                        ))
                    })
                    .category_context(|category| {
                        Some(widget::menu::items(
                            &HashMap::new(),
                            vec![
                                widget::menu::Item::Button(
                                    format!("Action on {} category", category.to_string()),
                                    None,
                                    MyAction::None,
                                ),
                                widget::menu::Item::Button(
                                    format!("Other action on {} category", category.to_string()),
                                    None,
                                    MyAction::None,
                                ),
                            ],
                        ))
                    })
                    .apply(Element::from)
            };

            if self.active_detail {
                self.screen_detail()
            } else {
                scrollable(table_wdget).into()
            }
        })
        .into()
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
