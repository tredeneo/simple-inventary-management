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

fn update_field<T>(
    target: &mut T,
    new_value: T,
) -> (Action<CreateUserMessage>, Task<CreateUserMessage>) {
    *target = new_value;
    (Action::None, Task::none())
}

#[derive(Debug, Clone)]
pub enum CreateUserMessage {
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
pub struct CreateUserPage {
    departments: combo_box::State<String>,
    name: String,
    department: Option<String>,
    email: String,
    ramal: String,
    celular: String,
    documento: String,
    login: String,
}

impl CreateUserPage {
    pub fn new() -> (Self, Task<CreateUserMessage>) {
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
            Action::App(CreateUserMessage::GetDepartments(tmp))
        });

        (app, command)
    }

    pub fn update(
        &mut self,
        message: Action<CreateUserMessage>,
    ) -> (Action<CreateUserMessage>, Task<CreateUserMessage>) {
        match message {
            Action::App(message) => match message {
                CreateUserMessage::CreateUser => {
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
                        Action::App(CreateUserMessage::CreatedUser(tmp))
                    });
                    (Action::None, task)
                }
                CreateUserMessage::GetDepartments(departs) => {
                    let mut tmp = Vec::new();

                    departs.iter().for_each(|f| tmp.push(f.name.clone()));
                    self.departments = combo_box::State::new(tmp);
                    (Action::None, Task::none())
                }
                CreateUserMessage::ChangingName(atual) => update_field(&mut self.name, atual),
                CreateUserMessage::ChangingDepartment(atual) => {
                    self.department = Some(atual);

                    (Action::None, Task::none())
                }
                CreateUserMessage::ChangingCelular(atual) => {
                    self.celular = atual;
                    (Action::None, Task::none())
                }
                CreateUserMessage::ChangingRamal(atual) => {
                    self.ramal = atual;
                    (Action::None, Task::none())
                }
                CreateUserMessage::ChangingDocumento(atual) => {
                    self.documento = atual;
                    (Action::None, Task::none())
                }
                CreateUserMessage::ChangingEmail(atual) => {
                    self.email = atual;
                    (Action::None, Task::none())
                }

                CreateUserMessage::ChangingLogin(atual) => {
                    self.login = atual;
                    (Action::None, Task::none())
                }
                CreateUserMessage::CreatedUser(result) => {
                    dbg!(result);

                    (Action::None, Task::none())
                }
            },
            _ => (Action::None, Task::none()),
        }
    }

    pub fn view(&self) -> Element<'_, CreateUserMessage> {
        let tmp = combo_box(
            &self.departments,
            "Selecione um departamento",
            self.department.as_ref(),
            CreateUserMessage::ChangingDepartment,
        );
        let department = row![text("departamento"), tmp];
        let tmp = text_input("", &self.name).on_input(CreateUserMessage::ChangingName);
        let name = row![text("name"), tmp];

        let tmp = text_input("", &self.email).on_input(CreateUserMessage::ChangingEmail);
        let email = row![text("email"), tmp];

        let tmp = text_input("", &self.documento).on_input(CreateUserMessage::ChangingDocumento);
        let documento = row![text("documento"), tmp];

        let tmp = text_input("", &self.ramal).on_input(CreateUserMessage::ChangingRamal);
        let ramal = row![text("ramal"), tmp];

        let tmp = text_input("", &self.login).on_input(CreateUserMessage::ChangingLogin);
        let login = row![text("login"), tmp];

        let tmp = text_input("escreva o celular ", &self.celular)
            .on_input(CreateUserMessage::ChangingCelular);
        let celular = row![text("celular"), tmp];
        let create = button("criar").on_press(CreateUserMessage::CreateUser);
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
