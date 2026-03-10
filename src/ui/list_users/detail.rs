use cosmic::Apply;
use std::collections::HashMap;

use cosmic::app::Task;
use cosmic::iced::widget::button;
use cosmic::iced::{self, Alignment, Length};
use cosmic::widget::{self, column, combo_box, row, scrollable, table, text, text_input};
use cosmic::{Action, Element};

use crate::database;

pub struct UserDetailPage {
    name: String,
    departments: combo_box::State<String>,
    department: Option<String>,
    email: String,
    ramal: String,
    celular: String,
    documento: String,
    login: String,
    equipaments: table::SingleSelectModel<Item, Category>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Category {
    #[default]
    Name,
    DataBegin,
    DataEnd,
    Serial,
}

#[derive(Default, Clone, Debug)]
struct Item {
    name: String,
    data_begin: String,
    data_end: String,
    serial: String,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Name => "Name",
            Self::DataBegin => "Data Begin",
            Self::DataEnd => "Data End",
            Self::Serial => "Serial",
        })
    }
}

impl table::ItemCategory for Category {
    fn width(&self) -> iced::Length {
        match self {
            Self::Name => iced::Length::Fixed(200.0),
            Self::Serial => iced::Length::Fill,
            Self::DataBegin => iced::Length::Fixed(150.),
            Self::DataEnd => iced::Length::Fixed(150.),
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
            Category::Serial => self.serial.clone().into(),
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
            Category::Serial => self.serial.to_uppercase().cmp(&other.serial.to_uppercase()),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UserDetailMessage {
    Close,
    Save,
    GetUserDetail(
        (
            database::model::DbUser,
            Vec<database::model::DbEquipamentHistoric>,
            Vec<database::model::DbDepartment>,
        ),
    ),
    ItemSelect(table::Entity),
    CategorySelect(Category),

    ChangingName(String),
    ChangingDepartment(String),
    ChangingCelular(String),
    ChangingRamal(String),
    ChangingDocumento(String),
    ChangingEmail(String),
    ChangingLogin(String),
    UserSaved(bool),
    NoOp,
}

impl UserDetailPage {
    pub fn init(user: String) -> (Self, Task<UserDetailMessage>) {
        let tmp = table::Model::new(vec![Category::Name, Category::DataBegin, Category::DataEnd]);
        let app = UserDetailPage {
            email: String::new(),
            ramal: String::new(),
            login: String::new(),
            department: Some(String::new()),
            documento: String::new(),
            celular: String::new(),
            departments: combo_box::State::new(Vec::new()),
            name: String::from("nome teste"),
            equipaments: tmp,
        };

        let command = Task::perform(
            async move {
                (
                    database::get_specific_user_by_name(user.clone()).await,
                    database::get_equipaments_by_users(user.clone()).await,
                    database::get_department().await,
                )
            },
            |(user, equipament, departments)| {
                dbg!(&user, &equipament);
                let tmp = user.unwrap_or_default();
                let equip = equipament.unwrap_or_default();
                let departments = departments.unwrap_or_default();
                cosmic::Action::App(UserDetailMessage::GetUserDetail((tmp, equip, departments)))
            },
        );
        (app, command)
    }

    pub fn update(&mut self, message: Action<UserDetailMessage>) -> Task<UserDetailMessage> {
        match message {
            Action::App(message) => match message {
                UserDetailMessage::GetUserDetail((user, equipaments, departments)) => {
                    let mut table_equipaments = table::Model::new(vec![
                        Category::Name,
                        Category::DataBegin,
                        Category::DataEnd,
                        Category::Serial,
                    ]);
                    equipaments.into_iter().for_each(|i| {
                        let tmp = Item {
                            name: i.model,
                            data_begin: i.initial_date,
                            data_end: i.final_date,
                            serial: i.serialnumber,
                        };
                        let _ = table_equipaments.insert(tmp);
                    });
                    self.name = user.name;
                    self.documento = user.document;
                    self.email = user.email;
                    self.login = user.login;
                    self.department = Some(user.department);
                    self.equipaments = table_equipaments;
                    self.departments =
                        combo_box::State::new(departments.into_iter().map(|f| f.name).collect());
                    self.celular = user.phone_number;
                    Task::none()
                }
                UserDetailMessage::ItemSelect(entinty) => {
                    self.equipaments.activate(entinty);
                    Task::none()
                }
                UserDetailMessage::CategorySelect(category) => {
                    let mut ascending = true;
                    if let Some(old_sort) = self.equipaments.get_sort()
                        && old_sort.0 == category
                    {
                        ascending = !old_sort.1;
                    }
                    self.equipaments.sort(category, ascending);
                    Task::none()
                }

                UserDetailMessage::ChangingName(atual) => {
                    self.name = atual;
                    Task::none()
                }

                UserDetailMessage::ChangingDepartment(atual) => {
                    self.department = Some(atual);

                    Task::none()
                }
                UserDetailMessage::ChangingCelular(atual) => {
                    self.celular = atual;
                    Task::none()
                }
                UserDetailMessage::ChangingRamal(atual) => {
                    self.ramal = atual;
                    Task::none()
                }
                UserDetailMessage::ChangingDocumento(atual) => {
                    self.documento = atual;
                    Task::none()
                }
                UserDetailMessage::ChangingEmail(atual) => {
                    self.email = atual;
                    Task::none()
                }

                UserDetailMessage::ChangingLogin(atual) => {
                    self.login = atual;
                    Task::none()
                }
                UserDetailMessage::Close => Task::none(),
                UserDetailMessage::Save => {
                    let tmp = database::model::DbUser {
                        name: self.name.clone(),
                        login: self.login.clone(),
                        email: self.email.clone(),
                        department: self.department.clone().unwrap_or_default(),
                        document: self.documento.clone(),
                        extension: self.ramal.clone(),
                        phone_number: self.celular.clone(),
                        ..Default::default()
                    };
                    let task = Task::perform(database::update_user(tmp), |tmp| {
                        let saved: bool;
                        if tmp.is_ok() {
                            saved = true;
                        } else {
                            saved = false;
                        }
                        cosmic::Action::App(UserDetailMessage::UserSaved(saved))
                    });
                    task
                }
                UserDetailMessage::UserSaved(_) => Task::none(),
                UserDetailMessage::NoOp => Task::none(),
            },
            _ => Task::none(),
        }
    }

    fn ui_table(&self) -> Element<'_, UserDetailMessage> {
        let table_widget = widget::table(&self.equipaments)
            .on_item_left_click(UserDetailMessage::ItemSelect)
            .on_category_left_click(UserDetailMessage::CategorySelect)
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
    pub fn view(&self) -> Element<'_, UserDetailMessage> {
        use cosmic::widget::container;
        let buttons = row()
            .push(button("back").on_press(UserDetailMessage::Close))
            .push(button("save").on_press(UserDetailMessage::Save));

        let text_width = 110;

        let tmp = combo_box(
            &self.departments,
            "Selecione um departamento",
            self.department.as_ref(),
            UserDetailMessage::ChangingDepartment,
        );
        let department = row()
            .push(container(text("departamento")).width(text_width))
            .push(tmp);

        let tmp = text_input("", &self.name).on_input(UserDetailMessage::ChangingName);
        let name = row()
            .push(container(text("name")).width(text_width))
            .push(tmp);

        let tmp = text_input("", &self.email).on_input(UserDetailMessage::ChangingEmail);
        let email = row()
            .push(container(text("email")).width(text_width))
            .push(tmp);

        let tmp = text_input("", &self.documento).on_input(UserDetailMessage::ChangingDocumento);
        let documento = row()
            .push(container(text("documento")).width(text_width))
            .push(tmp);

        let tmp = text_input("", &self.celular).on_input(UserDetailMessage::ChangingCelular);
        let celular = row()
            .push(container(text("celular")).width(text_width))
            .push(tmp);

        let tmp = text_input("", &self.login).on_input(UserDetailMessage::ChangingLogin);
        let login = row()
            .push(container(text("login")).width(text_width))
            .push(tmp);

        let coluna = column()
            .push(buttons)
            .push(text(format!("{}", self.name)).size(32))
            .push(name)
            .push(login)
            .push(email)
            .push(documento)
            .push(celular)
            .push(department)
            .push(self.ui_table());
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
    type Message = UserDetailMessage;

    fn message(&self) -> Self::Message {
        UserDetailMessage::NoOp
    }
}
