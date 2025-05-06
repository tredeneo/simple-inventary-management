pub mod database;
pub mod ui;

use crate::ui::counter::CounterMessage;
// use crate::ui::list_users::UsersMessage;

#[derive(Clone, Debug)]
pub enum Message {
    Counter(CounterMessage),
    // ListUsers(UsersMessage),
}
