use cosmic::{
    Action, Element,
    iced::{
        Alignment, Length,
        widget::{button, column, row},
    },
    iced_widget::text_input,
    widget::{container, text},
};

use crate::{Message, database};
use cosmic::app::Task;

#[derive(Debug, Clone)]
pub enum DepartmentsMessage {
    GetDepartments(Vec<database::model::DbDepartment>),
    CreateDepartment,
    ChangingName(String),
    DepartmentCreated(bool),
}

#[derive(Default)]
pub struct DepartmentsTab {
    departments: Vec<database::model::DbDepartment>,
    department: String,
}

impl DepartmentsTab {
    pub fn new() -> (Self, Task<DepartmentsMessage>) {
        let app = Self {
            departments: Vec::new(),
            department: String::new(),
        };
        let command = Task::perform(database::get_department(), |arg| {
            let tmp = arg.unwrap_or_default();
            Action::App(DepartmentsMessage::GetDepartments(tmp))
        });
        (app, command)
    }

    pub fn update(&mut self, message: DepartmentsMessage) -> Task<DepartmentsMessage> {
        match message {
            DepartmentsMessage::GetDepartments(departments) => {
                self.departments = departments;
                Task::none()
            }
            DepartmentsMessage::CreateDepartment => {
                let command = Task::perform(
                    database::insert_department(self.department.clone()),
                    |arg| {
                        let result = match arg {
                            Ok(_) => true,
                            Err(_) => false,
                        };
                        Action::App(DepartmentsMessage::DepartmentCreated(result))
                    },
                );
                command
            }
            DepartmentsMessage::DepartmentCreated(criado) => {
                if criado {
                    let command = Task::perform(database::get_department(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(DepartmentsMessage::GetDepartments(tmp))
                    });
                    return command;
                }

                return Task::none();
            }
            DepartmentsMessage::ChangingName(name) => {
                self.department = name;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, DepartmentsMessage> {
        let department_text = text_input("Novo departamento", &self.department)
            .on_input(DepartmentsMessage::ChangingName);
        let department_button =
            button("criar departmento").on_press(DepartmentsMessage::CreateDepartment);
        let create_department: Element<'_, DepartmentsMessage> =
            row![department_text, department_button].into();
        let department_list: Vec<Element<'_, DepartmentsMessage>> = self
            .departments
            .iter()
            .map(|department| Element::from(text(format!("departament:{}", department.name))))
            .collect();
        let content = column![create_department]
            .push(column(department_list).spacing(8))
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
