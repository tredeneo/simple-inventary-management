use cosmic::{
    Action, Element,
    iced::{
        Alignment, Length,
        widget::{button, column, row},
    },
    widget::{combo_box, container, text, text_input},
};

use crate::database::{self, model::DbUser};
use cosmic::app::Task;

fn update_field<T>(target: &mut T, new_value: T) -> Task<CreateModelMessage> {
    *target = new_value;
    Task::none()
}

#[derive(Debug, Clone)]
pub enum CreateModelMessage {
    GetDepartments(Vec<database::model::DbDepartment>),
    ChangingName(String),
    ChangingDepartment(String),
    ChangingCelular(String),
    ChangingRamal(String),
    ChangingDocumento(String),
    ChangingEmail(String),
    ChangingLogin(String),
    CreateUser,
    CreatedUser(bool),
}

#[derive(Debug, Default, Clone)]
pub struct CreateModelPage {
    departments: combo_box::State<String>,
    name: String,
    department: Option<String>,
    email: String,
    ramal: String,
    celular: String,
    documento: String,
    login: String,
}

impl CreateModelPage {
    pub fn new() -> (Self, Task<CreateModelMessage>) {
        let app = Self {
            departments: combo_box::State::new(Vec::new()),
            name: String::new(),
            email: String::new(),
            ramal: String::new(),
            celular: String::new(),
            documento: String::new(),
            login: String::new(),
            department: Some(String::new()),
        };

        let command = Task::perform(database::get_department(), |arg| {
            let tmp = arg.unwrap_or_default();
            Action::App(CreateModelMessage::GetDepartments(tmp))
        });

        (app, command)
    }

    pub fn update(&mut self, message: Action<CreateModelMessage>) -> Task<CreateModelMessage> {
        match message {
            Action::App(message) => match message {
                CreateModelMessage::CreateUser => {
                    let tmp = DbUser {
                        login: self.login.clone(),
                        name: self.name.clone(),
                        email: self.email.clone(),
                        department: self.department.clone().unwrap_or_default(),
                        document: self.documento.clone(),
                        id: 0,
                        extension: self.ramal.clone(),
                        phone_number: self.celular.clone(),
                    };
                    dbg!(&tmp);
                    let task = Task::perform(database::create_user(tmp), |arg| {
                        let tmp = match arg {
                            Ok(_) => true,
                            Err(_) => false,
                        };
                        Action::App(CreateModelMessage::CreatedUser(tmp))
                    });
                    task
                }
                CreateModelMessage::GetDepartments(departs) => {
                    let mut tmp = Vec::new();

                    departs.iter().for_each(|f| tmp.push(f.name.clone()));
                    self.departments = combo_box::State::new(tmp);
                    Task::none()
                }
                CreateModelMessage::ChangingName(atual) => update_field(&mut self.name, atual),
                CreateModelMessage::ChangingDepartment(atual) => {
                    self.department = Some(atual);

                    Task::none()
                }
                CreateModelMessage::ChangingCelular(atual) => {
                    self.celular = atual;
                    Task::none()
                }
                CreateModelMessage::ChangingRamal(atual) => {
                    self.ramal = atual;
                    Task::none()
                }
                CreateModelMessage::ChangingDocumento(atual) => {
                    self.documento = atual;
                    Task::none()
                }
                CreateModelMessage::ChangingEmail(atual) => {
                    self.email = atual;
                    Task::none()
                }

                CreateModelMessage::ChangingLogin(atual) => {
                    self.login = atual;
                    Task::none()
                }
                CreateModelMessage::CreatedUser(result) => {
                    dbg!(result);

                    Task::none()
                }
            },
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, CreateModelMessage> {
        let tmp = combo_box(
            &self.departments,
            "Selecione um departamento",
            self.department.as_ref(),
            CreateModelMessage::ChangingDepartment,
        );
        let department = row![text("departamento"), tmp];
        let tmp = text_input("", &self.name).on_input(CreateModelMessage::ChangingName);
        let name = row![text("name"), tmp];

        let tmp = text_input("", &self.email).on_input(CreateModelMessage::ChangingEmail);
        let email = row![text("email"), tmp];

        let tmp = text_input("", &self.documento).on_input(CreateModelMessage::ChangingDocumento);
        let documento = row![text("documento"), tmp];

        let tmp = text_input("", &self.ramal).on_input(CreateModelMessage::ChangingRamal);
        let ramal = row![text("ramal"), tmp];

        let tmp = text_input("", &self.login).on_input(CreateModelMessage::ChangingLogin);
        let login = row![text("login"), tmp];

        let tmp = text_input("escreva o celular ", &self.celular)
            .on_input(CreateModelMessage::ChangingCelular);
        let celular = row![text("celular"), tmp];
        let create = button("criar").on_press(CreateModelMessage::CreateUser);
        let content = column![
            department, login, name, documento, email, ramal, celular, create
        ]
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
