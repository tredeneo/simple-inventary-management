pub mod database;
pub mod ui;

use cosmic::Action;

use crate::ui::{
    brand::BrandsMessage, counter::CounterMessage, cpu::CPUsMessage,
    departments::DepartmentsMessage, equipament_models::EquipamentModelsMessage, gpu::GPUsMessage,
    list_users::UsersTabMessage,
};
// use crate::ui::list_users::list::UsersMessage;

#[derive(Clone, Debug)]
pub enum Message {
    Counter(CounterMessage),
    Users(Action<UsersTabMessage>),
    Departments(Action<DepartmentsMessage>),
    Brands(Action<BrandsMessage>),
    Cpus(Action<CPUsMessage>),
    Gpus(Action<GPUsMessage>),
    EquipamentModels(Action<EquipamentModelsMessage>),
}
