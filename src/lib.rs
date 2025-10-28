pub mod database;
pub mod ui;

use cosmic::Action;

use crate::ui::{
    counter::CounterMessage, departments::DepartmentsMessage, list_users::UsersTabMessage,
};
// use crate::ui::list_users::list::UsersMessage;

#[derive(Clone, Debug)]
pub enum Message {
    Counter(CounterMessage),
    Users(Action<UsersTabMessage>),
    Departments(Action<DepartmentsMessage>),
}
