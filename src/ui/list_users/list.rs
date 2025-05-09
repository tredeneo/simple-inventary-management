use cosmic::app::Task;
use cosmic::iced;
use cosmic::prelude::*;
use cosmic::widget::table;
use cosmic::widget::{self, nav_bar};

use crate::Message;

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
            Self::Name => iced::Length::Fill,
            Self::Department => iced::Length::Fixed(200.0),
            Self::Email => iced::Length::Fixed(150.0),
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

// #[derive(Clone, Debug)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UsersMessage {
    ItemSelect(table::Entity),
    CategorySelect(Category),
    NoOp,
}

pub struct ListUserTab {
    table_model: table::SingleSelectModel<Item, Category>,
}

impl ListUserTab {
    pub fn init() -> (Self, Task<UsersMessage>) {
        let mut nav_model = nav_bar::Model::default();

        nav_model.activate_position(0);

        let mut table_model =
            table::Model::new(vec![Category::Name, Category::Department, Category::Email]);

        // let _ = table_model.insert(Item {
        //     name: "Foo".into(),
        //     date: chrono::DateTime::default()
        //         .with_day(1)
        //         .unwrap()
        //         .with_month(1)
        //         .unwrap()
        //         .with_year(1970)
        //         .unwrap(),
        //     size: 2,
        // });

        let app = ListUserTab { table_model };

        let command = Task::none();

        (app, command)
    }

    pub fn update(&mut self, message: UsersMessage) -> Task<Message> {
        match message {
            UsersMessage::ItemSelect(entity) => self.table_model.activate(entity),
            UsersMessage::CategorySelect(category) => {
                let mut ascending = true;
                if let Some(old_sort) = self.table_model.get_sort() {
                    if old_sort.0 == category {
                        ascending = !old_sort.1;
                    }
                }
                self.table_model.sort(category, ascending)
            }
            UsersMessage::NoOp => {}
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, UsersMessage> {
        cosmic::widget::responsive(|size| {
            if size.width < 600.0 {
                widget::compact_table(&self.table_model)
                    .on_item_left_click(UsersMessage::ItemSelect)
                    .apply(Element::from)
            } else {
                widget::table(&self.table_model)
                    .on_item_left_click(UsersMessage::ItemSelect)
                    .on_category_left_click(UsersMessage::CategorySelect)
                    .apply(Element::from)
            }
        })
        .into()
    }
}

enum Action {
    None,
    Run(Task<UsersMessage>),
}
