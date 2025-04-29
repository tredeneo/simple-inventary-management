use iced::theme::Theme;
use iced::widget::{Container, button, column, text, text_input};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub struct UserDetail {
    pub name: String,
    pub department: String,
}

#[derive(Debug, Clone)]
pub enum UserDetailMessage {
    NameChanged(String),
    DepartmentChanged(String),
    Save,
    Cancel,
}

impl UserDetail {
    pub fn new(name: String, department: String) -> Self {
        Self { name, department }
    }

    pub fn update(&mut self, message: UserDetailMessage) -> Option<UserDetailMessage> {
        match message {
            UserDetailMessage::NameChanged(new) => {
                self.name = new;
                None
            }
            UserDetailMessage::DepartmentChanged(new) => {
                self.department = new;
                None
            }
            UserDetailMessage::Save => {
                // Aqui você poderia montar um Task, etc.
                Some(UserDetailMessage::Save)
            }
            UserDetailMessage::Cancel => Some(UserDetailMessage::Cancel),
        }
    }

    pub fn view(&self) -> Element<'_, UserDetailMessage> {
        column![
            text("Edit User").size(24),
            text_input("Name", &self.name)
                .on_input(UserDetailMessage::NameChanged)
                .padding(10),
            text_input("Department", &self.department)
                .on_input(UserDetailMessage::DepartmentChanged)
                .padding(10),
            button("Save").on_press(UserDetailMessage::Save),
            button("Cancel").on_press(UserDetailMessage::Cancel),
        ]
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
    }
}
