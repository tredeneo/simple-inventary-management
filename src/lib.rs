pub mod database;
pub mod ui;

use cosmic::Action;

use crate::ui::{
    brand::BrandsMessage, cpu::CPUsMessage, departments::DepartmentsMessage,
    equipament_models::EquipamentModelsMessage, gpu::GPUsMessage,
    list_equipaments::EquipamentListMessage, list_users::UsersTabMessage,
};

#[derive(Clone, Debug)]
pub enum Message {
    Users(Action<UsersTabMessage>),
    Departments(Action<DepartmentsMessage>),
    Brands(Action<BrandsMessage>),
    Cpus(Action<CPUsMessage>),
    Gpus(Action<GPUsMessage>),
    EquipamentModels(Action<EquipamentModelsMessage>),
    Equipaments(Action<EquipamentListMessage>),
}
