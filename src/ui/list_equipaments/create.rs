use cosmic::{
    Action, Element,
    iced::{
        Alignment, Length,
        widget::{button, column, row},
    },
    widget::{combo_box, container, text, text_input},
};

use crate::database::{self, model::DbComputer};
use cosmic::app::Task;

fn update_field<T>(target: &mut T, new_value: T) -> Task<CreateModelMessage> {
    *target = new_value;
    Task::none()
}

#[derive(Debug, Clone)]
pub enum CreateModelMessage {
    GetInformation(Vec<database::model::DbEquipamentModel>),
    ChangingSerialNumber(String),
    ChangingMemory(String),
    ChangingObservation(String),
    ChangingStorage(String),
    ChangingComputer(String),
    CreateUser,
    CreatedUser(bool),
}

#[derive(Debug, Default, Clone)]
pub struct CreateModelPage {
    serialnumber: String,
    computers: combo_box::State<String>,
    computer: Option<String>,
    storage: i32,
    memory: i32,
    observation: String,
}

impl CreateModelPage {
    pub fn new() -> (Self, Task<CreateModelMessage>) {
        let app = Self {
            serialnumber: String::new(),
            computers: combo_box::State::new(Vec::new()),
            computer: Some(String::new()),
            storage: 0,
            memory: 0,
            observation: String::new(),
        };

        let command = Task::perform(database::get_equipament_model(), |computers| {
            Action::App(CreateModelMessage::GetInformation(
                computers.unwrap_or_default(),
            ))
        });

        (app, command)
    }

    pub fn update(&mut self, message: Action<CreateModelMessage>) -> Task<CreateModelMessage> {
        match message {
            Action::App(message) => match message {
                CreateModelMessage::CreateUser => {
                    let tmp = DbComputer {
                        serialnumber: self.serialnumber.trim().to_string(),
                        memory: self.memory,
                        storage: self.storage,
                        model: self.computer.clone().unwrap_or_default().trim().to_string(),
                        observation: String::new(),
                        actual_user: String::new(),
                    };
                    dbg!(&tmp);
                    let task = Task::perform(database::create_computer(tmp), |arg| {
                        let tmp = match arg {
                            Ok(_) => true,
                            Err(_) => false,
                        };
                        Action::App(CreateModelMessage::CreatedUser(tmp))
                    });
                    task
                }
                CreateModelMessage::GetInformation(computer) => {
                    let mut tmp = Vec::with_capacity(computer.len());

                    computer.into_iter().for_each(|f| tmp.push(f.name));
                    self.computers = combo_box::State::new(tmp);

                    Task::none()
                }
                CreateModelMessage::ChangingSerialNumber(atual) => {
                    update_field(&mut self.serialnumber, atual.trim().to_string())
                }
                CreateModelMessage::ChangingMemory(atual) => {
                    let tmp = match atual.parse::<i32>() {
                        Ok(n) => n,
                        Err(..) => 0,
                    };
                    self.memory = tmp;

                    Task::none()
                }
                CreateModelMessage::ChangingStorage(atual) => {
                    let tmp = match atual.parse::<i32>() {
                        Ok(n) => n,
                        Err(e) => {
                            dbg!(e);
                            0
                        }
                    };
                    self.storage = tmp;
                    Task::none()
                }
                CreateModelMessage::ChangingComputer(atual) => {
                    self.computer = Some(atual);
                    Task::none()
                }

                CreateModelMessage::ChangingObservation(atual) => {
                    self.observation = atual;
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
        let size = 120;
        let tmp = combo_box(
            &self.computers,
            "Selecione a marca",
            self.computer.as_ref(),
            CreateModelMessage::ChangingComputer,
        );
        let computers =
            row![container(text("computers")).width(size), tmp].align_y(Alignment::Center);

        let tmp =
            text_input("", &self.serialnumber).on_input(CreateModelMessage::ChangingSerialNumber);
        let serial_number = row![container(text("serial number")).width(size), tmp];

        let tmp =
            text_input("", self.memory.to_string()).on_input(CreateModelMessage::ChangingMemory);
        let memory = row![container(text("memory")).width(size), tmp];

        let tmp = text_input("", self.observation.clone())
            .on_input(CreateModelMessage::ChangingObservation);
        let observation = row![container(text("observação")).width(size), tmp];

        let tmp =
            text_input("", self.storage.to_string()).on_input(CreateModelMessage::ChangingStorage);
        let storage = row![container(text("storage")).width(size), tmp];

        let create = button("criar").on_press(CreateModelMessage::CreateUser);
        let content = column![
            serial_number,
            computers,
            storage,
            memory,
            observation,
            create
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
