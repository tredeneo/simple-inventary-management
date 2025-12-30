use std::collections::HashMap;

use cosmic::{
    Action, Apply, Element,
    iced::{
        self, Alignment, Length,
        widget::{button, column, row},
    },
    iced_widget::text_input,
    widget::{self, container, scrollable, table},
};

use crate::database;
use cosmic::app::Task;

#[derive(Debug, Clone)]
pub enum DepartmentsMessage {
    GetDepartments(Vec<database::model::DbDepartment>),
    CreateDepartment,
    ChangingName(String),
    DepartmentCreated(bool),
    DeleteDepartment,
    DepartmentDeleted(bool),
    NoOp,
    ItemSelect(table::Entity),
}

pub struct DepartmentsTab {
    departments: table::SingleSelectModel<Item, Category>,
    department: String,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
enum Category {
    #[default]
    Name,
}

#[derive(Default, Debug, Clone)]
struct Item {
    name: String,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TESTE")
    }
}

impl table::ItemCategory for Category {
    fn width(&self) -> iced::Length {
        match self {
            Self::Name => iced::Length::Fixed(300.0),
        }
    }
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
        }
    }

    fn compare(&self, other: &Self, category: Category) -> std::cmp::Ordering {
        match category {
            Category::Name => self.name.to_lowercase().cmp(&other.name.to_lowercase()),
        }
    }
}
impl DepartmentsTab {
    pub fn new() -> (Self, Task<DepartmentsMessage>) {
        let table_model = table::Model::new(vec![Category::Name]);
        let app = Self {
            departments: table_model,
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
                let mut table_mode = table::Model::new(vec![]);

                departments.into_iter().for_each(|i| {
                    let tmp = Item { name: i.name };
                    let _ = table_mode.insert(tmp);
                });
                self.departments = table_mode;
                Task::none()
            }
            DepartmentsMessage::ItemSelect(entity) => {
                dbg!(entity);
                self.departments.activate(entity);
                let tmp = self.departments.active();
                dbg!(tmp);
                let department: Option<&String> = self.departments.data::<String>(tmp);
                dbg!(department);
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
            DepartmentsMessage::DeleteDepartment => {
                dbg!("delete");
                let department: Option<&Item> = self.departments.active_data();
                dbg!(department);
                if department.is_none() {
                    return Task::none();
                }
                let name = department.unwrap();
                dbg!(&name);
                let command =
                    Task::perform(database::delete_department(name.name.clone()), |arg| {
                        let result = match arg {
                            Ok(_) => true,
                            Err(_) => false,
                        };
                        Action::App(DepartmentsMessage::DepartmentDeleted(result))
                    });
                command
            }
            DepartmentsMessage::DepartmentDeleted(deleteado) => {
                if deleteado {
                    let command = Task::perform(database::get_department(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(DepartmentsMessage::GetDepartments(tmp))
                    });
                    return command;
                }

                return Task::none();
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
            DepartmentsMessage::NoOp => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, DepartmentsMessage> {
        let department_text = text_input("Novo departamento", &self.department)
            .on_input(DepartmentsMessage::ChangingName);
        let department_create =
            button("criar departmento").on_press(DepartmentsMessage::CreateDepartment);
        let department_delete =
            button("excluir departmento").on_press(DepartmentsMessage::DeleteDepartment);
        let create_department: Element<'_, DepartmentsMessage> =
            row![department_text, department_create, department_delete].into();
        let department_list = widget::compact_table(&self.departments)
            .on_item_left_click(DepartmentsMessage::ItemSelect)
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
            .apply(Element::from);
        let scrol = scrollable(department_list).into();
        let content = column![create_department]
            .push(column(vec![scrol]).spacing(8))
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum MyAction {
    None,
}

impl widget::menu::Action for MyAction {
    type Message = DepartmentsMessage;

    fn message(&self) -> Self::Message {
        DepartmentsMessage::NoOp
    }
}
