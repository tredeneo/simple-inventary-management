use std::collections::HashMap;

use cosmic::iced::{self, Alignment, Background, Color, Length};
use cosmic::widget::{
    self as widget, button, column, container, row, scrollable, table, text_input,
};
use cosmic::{Action, Apply, Element};

use crate::database;
use cosmic::app::Task;

#[derive(Debug, Clone)]
pub enum DepartmentsMessage {
    // LoadedDepartments(Vec<database::model::DbDepartment>),
    GetDepartments(Vec<database::model::DbDepartment>),
    CreateDepartment,
    ChangingName(String),
    DepartmentCreated(bool),
    DeleteDepartment,
    DepartmentDeleted(bool),
    ItemSelect(table::Entity),
    OpenCreateModal,
    CloseCreateModal,
}

pub struct DepartmentsTab {
    departments: table::SingleSelectModel<Item, Category>,
    department: String,
    show_create_modal: bool,
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
        f.write_str(match self {
            Self::Name => "Name",
        })
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
    fn get_icon(&self, _category: Category) -> Option<cosmic::widget::Icon> {
        None
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

            show_create_modal: false,
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
            }
            DepartmentsMessage::ItemSelect(entity) => self.departments.activate(entity),

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
                return command;
            }
            DepartmentsMessage::DeleteDepartment => {
                let department = self
                    .departments
                    .item(self.departments.active())
                    .cloned()
                    .unwrap_or_default()
                    .name;

                let command =
                    Task::perform(database::delete_department(department.clone()), |arg| {
                        let result = match arg {
                            Ok(_) => true,
                            Err(_) => false,
                        };
                        Action::App(DepartmentsMessage::DepartmentDeleted(result))
                    });
                return command;
            }
            DepartmentsMessage::DepartmentDeleted(deleteado) => {
                if deleteado {
                    let command = Task::perform(database::get_department(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(DepartmentsMessage::GetDepartments(tmp))
                    });
                    return command;
                }
            }
            DepartmentsMessage::DepartmentCreated(criado) => {
                if criado {
                    let command = Task::perform(database::get_department(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(DepartmentsMessage::GetDepartments(tmp))
                    });
                    self.show_create_modal = false;
                    return command;
                }
            }
            DepartmentsMessage::ChangingName(name) => {
                self.department = name;
            }

            DepartmentsMessage::OpenCreateModal => {
                self.show_create_modal = true;
            }
            DepartmentsMessage::CloseCreateModal => {
                self.show_create_modal = false;
            }
        };
        Task::none()
    }

    pub fn view(&self) -> Element<'_, DepartmentsMessage> {
        let department_delete =
            button::text("criar marca").on_press(DepartmentsMessage::OpenCreateModal);

        let create_department: Element<'_, DepartmentsMessage> = row()
            .push(department_delete)
            .align_y(Alignment::Center)
            .into();

        let department_list = widget::compact_table(&self.departments)
            .on_item_left_click(DepartmentsMessage::ItemSelect)
            .item_context(|item| {
                Some(widget::menu::items(
                    &HashMap::new(),
                    vec![widget::menu::Item::Button(
                        format!("Excluir Department on {}", item.name),
                        None,
                        MyAction::Selecionado,
                    )],
                ))
            })
            .on_item_right_click(DepartmentsMessage::ItemSelect)
            .apply(Element::from);

        let scrol = scrollable(department_list);

        let content = column()
            .push(create_department)
            .push(column().push(scrol).spacing(8))
            .spacing(16)
            .padding(20)
            .max_width(600);

        let base = container(content)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        if self.show_create_modal {
            let department_input = text_input("Department name", &self.department)
                .on_input(DepartmentsMessage::ChangingName);

            let actions = row()
                .push(button::text("Cancel").on_press(DepartmentsMessage::CloseCreateModal))
                .push(button::text("Create").on_press(DepartmentsMessage::CreateDepartment))
                .spacing(8);

            let modal_content = container(
                column()
                    .push(department_input)
                    .push(actions)
                    .spacing(12)
                    .padding(20)
                    .width(Length::Fixed(400.0)),
            )
            .style(|theme: &cosmic::Theme| {
                let cosmic = theme.cosmic();
                iced::widget::container::Style {
                    background: Some(Background::Color(Color::from(cosmic.primary.base))),
                    text_color: Some(Color::from(cosmic.primary.on)),
                    ..Default::default()
                }
            });

            let tmp = widget::popover(base)
                .modal(true)
                .position(widget::popover::Position::Center)
                .on_close(DepartmentsMessage::CloseCreateModal);

            return tmp.popup(modal_content).into();
        }

        base.into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MyAction {
    Selecionado,
}

impl widget::menu::Action for MyAction {
    type Message = DepartmentsMessage;

    fn message(&self) -> Self::Message {
        DepartmentsMessage::DeleteDepartment
    }
}
