use cosmic::Apply;
use cosmic::iced_widget::scrollable;
use std::collections::HashMap;

use cosmic::app::Task;
use cosmic::iced::widget::button;
use cosmic::iced::{self, Alignment, Length};
use cosmic::widget::{self, combo_box, table, text};
use cosmic::widget::{column, container, row};
use cosmic::{Action, Element};

use crate::{database, popup_style};

pub struct EquipamentDetailPage {
    serial: String,
    model: String,
    memory: i32,
    storage: i32,
    observation: String,
    users_historic: table::SingleSelectModel<Item, Category>,
    users: combo_box::State<String>,
    future_user: Option<String>,
    actual_user: String,
    page: Page,
}

#[derive(Debug)]
enum Page {
    Detail,
    ChangeUser,
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
    OpenCreateModal,
    ChangeUser(String),
    GetUsers(Vec<database::model::DbUser>),
    UserChanged(bool),
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
            users_historic: tmp,
            actual_user: String::new(),
            future_user: Some(String::new()),
            users: combo_box::State::new(Vec::new()),
            page: Page::Detail,
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
    ) -> Task<EquipamentDetailMessage> {
        match message {
            Action::App(message) => match message {
                EquipamentDetailMessage::InfoUpdated((users, computer)) => {
                    let mut table_users = table::Model::new(vec![
                        Category::Name,
                        Category::DataBegin,
                        Category::DataEnd,
                    ]);
                    users.into_iter().for_each(|i| {
                        let tmp = Item {
                            name: i.usuario,
                            data_begin: i.date_begin,
                            data_end: i.date_end.unwrap_or_default(),
                        };
                        let _ = table_users.insert(tmp);
                    });

                    self.serial = computer.serialnumber;
                    self.model = computer.model;
                    self.memory = computer.memory;
                    self.observation = computer.observation;
                    self.storage = computer.storage;
                    self.users_historic = table_users;
                    self.actual_user = computer.actual_user;
                }

                EquipamentDetailMessage::ItemSelect(entity) => {
                    self.users_historic.activate(entity);
                }
                EquipamentDetailMessage::CategorySelect(category) => {
                    let mut ascending = true;
                    if let Some(old_sort) = self.users_historic.get_sort()
                        && old_sort.0 == category
                    {
                        ascending = !old_sort.1;
                    }
                    self.users_historic.sort(category, ascending);
                }
                EquipamentDetailMessage::OpenCreateModal => {
                    let command = Task::perform(database::get_users(), |users_list| {
                        if users_list.is_err() {
                            return Action::None;
                        }
                        Action::App(EquipamentDetailMessage::GetUsers(users_list.unwrap()))
                    });
                    self.page = Page::ChangeUser;
                    return command;
                }
                EquipamentDetailMessage::GetUsers(users) => {
                    let mut tmp = Vec::new();
                    users.iter().for_each(|i| tmp.push(i.name.clone()));
                    self.users = combo_box::State::new(tmp);
                }
                EquipamentDetailMessage::ChangeUser(actual) => self.future_user = Some(actual),
                EquipamentDetailMessage::Close => {
                    self.page = Page::Detail;
                }
                EquipamentDetailMessage::Save => {
                    let command = Task::perform(
                        database::update_user_equipament(
                            self.actual_user.clone(),
                            self.future_user.clone().unwrap_or_default(),
                            self.serial.clone(),
                        ),
                        |arg| {
                            let tmp = match arg {
                                Ok(_) => true,
                                Err(_) => false,
                            };
                            Action::App(EquipamentDetailMessage::UserChanged(tmp))
                        },
                    );
                    return command;
                }
                EquipamentDetailMessage::UserChanged(changed) => {
                    if changed {
                        self.page = Page::Detail;
                    }
                }
                EquipamentDetailMessage::NoOp => {
                    dbg!("NoOp");
                }
            },

            _ => return Task::none(),
        };
        Task::none()
    }
    fn ui_table(&self) -> Element<'_, EquipamentDetailMessage> {
        let table_widget = widget::table(&self.users_historic)
            .on_item_left_click(EquipamentDetailMessage::ItemSelect)
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
        scrollable(table_widget).into()
    }

    pub fn ui_detail(&self) -> Element<'_, EquipamentDetailMessage> {
        let buttons = row()
            .push(button("back").on_press(EquipamentDetailMessage::Close))
            .push(button("trocar").on_press(EquipamentDetailMessage::OpenCreateModal));
        let coluna = column()
            .push(buttons)
            .push(text(format!(" teste {}", self.model)).size(32))
            .push(self.ui_table());

        container(coluna)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }

    pub fn ui_change_user(&self) -> Element<'_, EquipamentDetailMessage> {
        let buttons = row()
            .push(button("back").on_press(EquipamentDetailMessage::Close))
            .push(button("save").on_press(EquipamentDetailMessage::Save));

        let user = combo_box(
            &self.users,
            "Select users",
            self.future_user.as_ref(),
            EquipamentDetailMessage::ChangeUser,
        );

        let content =
            container(column().push(user).push(buttons).padding(20).width(400)).style(popup_style);
        widget::popover(self.ui_detail())
            .modal(true)
            .position(widget::popover::Position::Center)
            .popup(content)
            .into()
    }

    pub fn view(&self) -> Element<'_, EquipamentDetailMessage> {
        match self.page {
            Page::Detail => self.ui_detail(),
            Page::ChangeUser => self.ui_change_user(),
        }
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
