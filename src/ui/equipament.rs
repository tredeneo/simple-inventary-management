use std::rc::Rc;

use crate::database::{self};
use crate::{App, ChangeEquipament, ComputerDetail, Computers};
use slint::{ComponentHandle, ModelRc, SharedString, StandardListViewItem, VecModel};

pub async fn equipament_list(app: &App) -> anyhow::Result<()> {
    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());
    let tmp = database::get_computers().await.unwrap_or_default();

    for i in tmp {
        // for i in tc
        let items = Rc::new(VecModel::default());
        items.push(slint::format!("{}", i.serialnumber).into());
        items.push(slint::format!("{}", i.actual_user).into());
        items.push(slint::format!("{}", i.model).into());
        row_data.push(items.into());
    }

    app.global::<Computers>()
        .set_row_data(row_data.clone().into());

    Ok(())
}

async fn get_equipament_model() -> anyhow::Result<ModelRc<SharedString>> {
    let cpus = database::get_equipament_model().await?;
    let mut row_data = Vec::default();
    for i in cpus {
        row_data.push(slint::format!("{}", i.name));
    }
    Ok(ModelRc::from(row_data.as_slice()))
}

async fn last_user(app: App, serial: &str) -> anyhow::Result<()> {
    // let row_data: Rc<VecModel<slint::VecModel<StandardListViewItem>>> =
    // Rc::new(VecModel::default());
    let row_data = Rc::new(VecModel::default());
    let local_app = app.clone_strong();
    let users = database::get_user_computers(&serial).await?;
    // dbg!(&serial);
    for i in users {
        let items = Rc::new(VecModel::default());
        items.push(slint::format!("{}", i.usuario).into());
        items.push(slint::format!("{}", i.date_begin).into());
        items.push(slint::format!("{}", i.date_end.unwrap_or_default()).into());
        row_data.push(items.into());
    }
    // let tmp = ModelRc::from(row_data.clone());
    app.global::<ComputerDetail>().set_row_data(row_data.into());
    Ok(())
}

pub async fn change_equipament(app: &App) -> anyhow::Result<()> {
    use crate::ui::user::get_user_list;

    let row_data = get_user_list().await?;
    app.global::<ChangeEquipament>()
        .set_users(row_data.clone().into());

    app.global::<ComputerDetail>()
        .set_model_equipaments(get_equipament_model().await?);

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
            equipament_list(&local_app).await.ok();
        })
        .ok();
    });
    Ok(())
}
