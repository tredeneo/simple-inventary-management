use std::rc::Rc;

use crate::database::{self};
use crate::{global_update, App, ChangeEquipament, ComputerDetail};
use slint::{ComponentHandle, VecModel};

async fn last_user(app: App, serial: &str) -> anyhow::Result<()> {
    let row_data = Rc::new(VecModel::default());
    let users = database::get_user_computers(serial).await?;
    for i in users {
        let items = Rc::new(VecModel::default());
        items.push(slint::format!("{}", i.usuario).into());
        items.push(slint::format!("{}", i.date_begin).into());
        items.push(slint::format!("{}", i.date_end.unwrap_or_default()).into());
        row_data.push(items.into());
    }
    app.global::<ComputerDetail>().set_row_data(row_data.into());
    Ok(())
}

pub async fn change_equipament(app: &App) -> anyhow::Result<()> {
    let myapp = app.clone_strong();
    app.global::<ComputerDetail>().on_users_history(move |arg| {
        let myapp = myapp.clone_strong();
        slint::spawn_local(async move {
            last_user(myapp, arg.as_str()).await.ok();
        })
        .ok();
    });
    let myapp = app.clone_strong();
    app.global::<ComputerDetail>().on_create_computer(move || {
        let local_app = myapp.clone_strong();

        let computer = local_app.global::<ComputerDetail>();
        let equipament = database::model::DbComputer {
            serialnumber: computer.get_serial_number().to_string(),
            memory: computer.get_memory().parse::<i32>().unwrap_or_default(),
            storage: computer.get_storage().parse::<i32>().unwrap_or_default(),
            observation: computer.get_observation().to_string(),
            actual_user: computer.get_actual_user().to_string(),
            model: computer.get_model_equipament().to_string(),
        };
        slint::spawn_local(async move {
            database::create_computer(equipament).await.ok();
            global_update(&local_app).await.ok();
        })
        .ok();
    });

    let myapp = app.clone_strong();
    app.global::<ChangeEquipament>().on_change_user(move || {
        let local_app = myapp.clone_strong();
        let computer = myapp.global::<ComputerDetail>();
        let serial = computer.get_serial_number();

        let actual_user = computer.get_actual_user();
        let future_user = local_app.global::<ChangeEquipament>().get_future_login();

        slint::spawn_local(async move {
            database::update_user_equipament(
                actual_user.to_string(),
                future_user.to_string(),
                serial.to_string(),
            )
            .await
            .ok();
            global_update(&local_app).await.ok();
        })
        .ok();
    });
    Ok(())
}
